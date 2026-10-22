//! Who is allowed to use this agent: a fixed allowlist, plus peers that enrolled with a shared token.
//!
//! With an enrollment token configured, any peer may connect, but a stranger can only fetch the Agent Card and call
//! `agents.enroll`; every other skill is refused until it has enrolled. Enrolling exchanges the token for access
//! for a limited time, so outside agents can appear and disappear (autoscaling) without the allowlist changing or this
//! agent restarting. Enrollments live in memory: a restart forgets them, and clients simply enroll again.
//!
//! Without a token, only the fixed allowlist may connect at all and nobody can enroll. This is built for a demo: one
//! shared token, no per-agent revocation. Rotate the token and restart to cut everyone off.
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
    time::{Duration, Instant},
};

use a2a_iroh::{iroh::EndpointId, NOT_ENROLLED};

use crate::skill::{SkillError, SkillOutput, SkillResult};

/// The most peers enrolled at once, so a flood of fresh keys cannot grow memory without bound.
const MAX_ENROLLED: usize = 200;

pub struct Enrollments {
    /// The shared secret, or `None` if enrollment is switched off.
    token: Option<String>,
    /// How long an enrollment lasts.
    lifetime: Duration,
    /// Peers that are always allowed, with no enrollment needed (`ALLOWED_PEERS`).
    fixed_allowlist: HashSet<EndpointId>,
    /// Enrolled peers and when each enrollment ends.
    enrolled_until: Mutex<HashMap<EndpointId, Instant>>,
}

impl Enrollments {
    pub fn new(token: Option<String>, lifetime: Duration, fixed_allowlist: HashSet<EndpointId>) -> Self {
        Self { token, lifetime, fixed_allowlist, enrolled_until: Mutex::new(HashMap::new()) }
    }

    pub fn is_enabled(&self) -> bool {
        self.token.is_some()
    }

    /// May this peer open a connection at all? With enrollment on, anyone may (they just cannot do much);
    /// with it off, only the fixed allowlist.
    pub fn may_connect(&self, peer: &EndpointId) -> bool {
        self.is_enabled() || self.fixed_allowlist.contains(peer)
    }

    /// May this peer use the skills? Yes if it is on the fixed allowlist or holds an unexpired enrollment.
    pub fn is_enrolled(&self, peer: &EndpointId) -> bool {
        if self.fixed_allowlist.contains(peer) {
            return true;
        }
        let enrolled = self.enrolled_until.lock().unwrap();
        enrolled.get(peer).is_some_and(|until| Instant::now() < *until)
    }

    /// The `agents.enroll` skill: trade the shared token for access for [`Self::lifetime`].
    pub fn enroll(&self, peer: &EndpointId, offered_token: &str) -> SkillResult {
        let Some(token) = &self.token else {
            return Err(rule("enrollment is not enabled on this agent"));
        };
        if self.fixed_allowlist.contains(peer) {
            return Ok(SkillOutput::new("already allowed", serde_json::json!({"enrolled": true})));
        }
        if !same_secret(token, offered_token) {
            return Err(rule("invalid enrollment token"));
        }

        let mut enrolled = self.enrolled_until.lock().unwrap();
        let now = Instant::now();
        enrolled.retain(|_, until| now < *until);
        if enrolled.len() >= MAX_ENROLLED && !enrolled.contains_key(peer) {
            return Err(rule("enrollment is full, try again later"));
        }
        enrolled.insert(*peer, now + self.lifetime);
        let hours = self.lifetime.as_secs() / 3600;
        Ok(SkillOutput::new(format!("enrolled for {hours} hours"), serde_json::json!({"enrolled": true})))
    }

    /// The refusal for a peer that is neither allowlisted nor enrolled. Its text contains [`NOT_ENROLLED`], which is how
    /// clients recognize that they should enroll.
    pub fn refusal() -> SkillError {
        rule(format!("{NOT_ENROLLED}: call agents.enroll with a valid token"))
    }
}

fn rule(message: impl Into<String>) -> SkillError {
    SkillError::RuleDenied(message.into())
}

/// Compare secrets without stopping at the first difference, so response time does not reveal how much matched.
fn same_secret(expected: &str, offered: &str) -> bool {
    let (expected, offered) = (expected.as_bytes(), offered.as_bytes());
    let length_matches = expected.len() == offered.len();
    let mut differences = 0u8;
    for (position, byte) in expected.iter().enumerate() {
        differences |= byte ^ offered.get(position).copied().unwrap_or(0);
    }
    length_matches && differences == 0
}

#[cfg(test)]
mod tests {
    use a2a_iroh::iroh::SecretKey;

    use super::*;

    fn peer() -> EndpointId {
        SecretKey::generate().public()
    }

    fn with_token(token: &str) -> Enrollments {
        Enrollments::new(Some(token.into()), Duration::from_secs(3600), HashSet::new())
    }

    #[test]
    fn a_stranger_is_not_enrolled_until_it_offers_the_right_token() {
        let enrollments = with_token("open-sesame");
        let newcomer = peer();
        assert!(enrollments.may_connect(&newcomer), "with enrollment on, anyone may connect");
        assert!(!enrollments.is_enrolled(&newcomer));

        assert!(matches!(enrollments.enroll(&newcomer, "guess"), Err(SkillError::RuleDenied(_))));
        assert!(matches!(enrollments.enroll(&newcomer, ""), Err(SkillError::RuleDenied(_))));
        assert!(
            matches!(enrollments.enroll(&newcomer, "open-sesam"), Err(SkillError::RuleDenied(_))),
            "a prefix is not enough"
        );
        assert!(!enrollments.is_enrolled(&newcomer), "failed attempts enroll nobody");

        enrollments.enroll(&newcomer, "open-sesame").unwrap();
        assert!(enrollments.is_enrolled(&newcomer));
        assert!(!enrollments.is_enrolled(&peer()), "enrolling one peer enrolls only that peer");
    }

    #[test]
    fn enrollments_expire() {
        let enrollments = Enrollments::new(Some("t".into()), Duration::from_millis(30), HashSet::new());
        let newcomer = peer();
        enrollments.enroll(&newcomer, "t").unwrap();
        assert!(enrollments.is_enrolled(&newcomer));
        std::thread::sleep(Duration::from_millis(60));
        assert!(!enrollments.is_enrolled(&newcomer), "an expired enrollment no longer counts");
    }

    #[test]
    fn without_a_token_only_the_fixed_allowlist_may_connect_and_nobody_can_enroll() {
        let friend = peer();
        let enrollments = Enrollments::new(None, Duration::from_secs(3600), HashSet::from([friend]));
        assert!(enrollments.may_connect(&friend) && enrollments.is_enrolled(&friend));
        let stranger = peer();
        assert!(!enrollments.may_connect(&stranger));
        assert!(enrollments.enroll(&stranger, "anything").is_err());
    }

    #[test]
    fn the_fixed_allowlist_still_works_alongside_enrollment() {
        let friend = peer();
        let enrollments = Enrollments::new(Some("t".into()), Duration::from_secs(3600), HashSet::from([friend]));
        assert!(enrollments.is_enrolled(&friend), "no enrollment needed");
        assert!(enrollments.enroll(&friend, "wrong").is_ok(), "an allowlisted peer is never turned away");
    }

    #[test]
    fn the_number_of_enrolled_peers_is_capped() {
        let enrollments = with_token("t");
        for _ in 0..MAX_ENROLLED {
            enrollments.enroll(&peer(), "t").unwrap();
        }
        assert!(enrollments.enroll(&peer(), "t").is_err(), "the next newcomer is refused");
    }

    #[test]
    fn secrets_are_compared_in_full() {
        assert!(same_secret("abc", "abc"));
        assert!(
            !same_secret("abc", "abd")
                && !same_secret("abc", "ab")
                && !same_secret("abc", "abcd")
                && !same_secret("abc", "")
        );
    }
}
