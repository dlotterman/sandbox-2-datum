# Software bill of materials

Generated 2026-10-21 with Syft 1.55.0, rustc 1.99.1 (58cf8f9c3 2026-10-13) and cargo 1.99.1 (5f94df478 2026-08-27). Regenerate with [`scripts/generate-sbom.py`](scripts/generate-sbom.py).

## Scope and provenance

This snapshot covers the Rust crates linked into the two executables (`inside-agent`, `outside-agent`, resolved by `cargo tree -e normal` from the committed lockfile) and the packages Syft detected in the three container images built from this repository's Containerfiles. Versions are the locked or observed versions.

- Crates linked into the executables: **274** (16 direct, 25 are procedural macros that run at build time only).
- `Cargo.lock` SHA256: `382d87f38031191fbac26a630ef6d4831d4c1bf8a9a8b3a0ff5b72e6d59e1ad2`.
- Workspace crates (this project, The Unlicense): `a2a-iroh`, `inside-agent`, `outside-agent`.
- Build-only and test-only crates (`-e normal` excludes them) and the Rust toolchain are not part of the executables and are not listed.
- The Containerfiles use mutable base tags (`debian:bookworm-slim`, `rclone/rclone:latest`), so rebuilding can change the operating-system package set. The image IDs below identify what was scanned.

## License interpretation

The Unlicense covers only this project's original code, documentation and packaging. Third-party software keeps its own licenses and notices. License identifiers below describe the component they are listed against; they are not a choice of license for the whole application.

Crate licenses are the SPDX expressions declared in each crate's `Cargo.toml` (for example `MIT OR Apache-2.0` means either license may be selected). The *Evidence* column lists license and notice files found in the crate's published source. This is package-level metadata, not a file-level audit of the source. Container license expressions are Syft's observations of package metadata and installed copyright files; `NOASSERTION` means it could not establish a license, not that the component is unlicensed.

## Crate license summary

Counts of linked crates mentioning each identifier (an `OR` expression counts once for each alternative).

| License identifier | Crates |
| --- | --- |
| MIT | 239 |
| Apache-2.0 | 183 |
| Unicode-3.0 | 20 |
| BSD-3-Clause | 8 |
| ISC | 6 |
| Zlib | 4 |
| BSD-2-Clause | 3 |
| CC0-1.0 | 2 |
| Unlicense | 2 |
| BSL-1.0 | 2 |
| MPL-2.0 | 1 |
| LLVM-exception | 1 |
| MIT-0 | 1 |
| CDLA-Permissive-2.0 | 1 |

## Worth calling out

Crates whose license expression includes something other than the common permissive set (0BSD, Apache-2.0, BSD-2-Clause, BSD-3-Clause, BSL-1.0, CC0-1.0, ISC, MIT, Unlicense, Zlib), declares no SPDX expression, or combines licenses with `AND`:

| Crate | Version | License | Evidence | Note |
| --- | --- | --- | --- | --- |
| attohttpc | 0.30.1 | MPL-2.0 | LICENSE | MPL-2.0 is file-level copyleft: changes to its own files must be shared; this project uses it unmodified. |
| blake3 | 1.8.7 | CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception | LICENSE_A2; LICENSE_A2LLVM; LICENSE_CC0 | Triple choice: CC0-1.0, Apache-2.0, or Apache-2.0 with the LLVM exception. |
| constant_time_eq | 0.4.2 | CC0-1.0 OR MIT-0 OR Apache-2.0 | LICENSE-APACHE; LICENSE-CC0; LICENSE-MIT0 |  |
| icu_collections | 2.3.0 | Unicode-3.0 | LICENSE | Unicode data license |
| icu_locale_core | 2.3.0 | Unicode-3.0 | LICENSE | Unicode data license |
| icu_normalizer | 2.3.0 | Unicode-3.0 | LICENSE | Unicode data license |
| icu_normalizer_data | 2.3.0 | Unicode-3.0 | LICENSE | Unicode data license |
| icu_properties | 2.3.0 | Unicode-3.0 | LICENSE | Unicode data license |
| icu_properties_data | 2.3.0 | Unicode-3.0 | LICENSE | Unicode data license |
| icu_provider | 2.3.1 | Unicode-3.0 | LICENSE | Unicode data license |
| litemap | 0.8.3 | Unicode-3.0 | LICENSE | Unicode data license |
| matchit | 0.8.4 | MIT AND BSD-3-Clause | LICENSE; LICENSE.httprouter | MIT AND BSD-3-Clause; includes code derived from httprouter (LICENSE.httprouter). |
| potential_utf | 0.1.6 | Unicode-3.0 | LICENSE | Unicode data license |
| ring | 0.17.14 | Apache-2.0 AND ISC | LICENSE; LICENSE-BoringSSL; LICENSE-other-bits | Declared Apache-2.0 AND ISC; the source also ships LICENSE-BoringSSL and LICENSE-other-bits, so it carries BoringSSL-derived code and third-party notices that must travel with redistributions. |
| tinystr | 0.8.4 | Unicode-3.0 | LICENSE | Unicode data license |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | LICENSE-APACHE; LICENSE-MIT; LICENSE-UNICODE | Unicode data under Unicode-3.0 in addition to MIT OR Apache-2.0. |
| unicode-normalization | 0.1.26 | (MIT OR Apache-2.0) AND Unicode-3.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT; LICENSE-UNICODE | Unicode data under Unicode-3.0 in addition to MIT OR Apache-2.0. |
| webpki-roots | 1.0.10 | CDLA-Permissive-2.0 | LICENSE | Mozilla CA root certificate data, licensed CDLA-Permissive-2.0. |
| writeable | 0.6.4 | Unicode-3.0 | LICENSE | Unicode data license |
| yoke | 0.8.3 | Unicode-3.0 | LICENSE | Unicode data license |
| yoke-derive | 0.8.4 | Unicode-3.0 | LICENSE | Unicode data license |
| zerofrom | 0.1.8 | Unicode-3.0 | LICENSE | Unicode data license |
| zerofrom-derive | 0.1.8 | Unicode-3.0 | LICENSE | Unicode data license |
| zerotrie | 0.2.5 | Unicode-3.0 | LICENSE | Unicode data license |
| zerovec | 0.11.8 | Unicode-3.0 | LICENSE | Unicode data license |
| zerovec-derive | 0.11.6 | Unicode-3.0 | LICENSE | Unicode data license |

Operating-system packages for which Syft found no license (it cannot tell us they are unlicensed; for example glibc is LGPL-2.1-or-later with exceptions):

| Image | Packages with NOASSERTION |
| --- | --- |
| inside-agent | base-files, debian-archive-keyring, gcc-12-base, libc-bin, libc6, libcrypt1, libgcc-s1, libgcrypt20, libgnutls30, libselinux1, libsemanage-common, libsemanage2 (+2 more, see the package list below) |
| outside-agent | base-files, debian-archive-keyring, gcc-12-base, libc-bin, libc6, libcrypt1, libgcc-s1, libgcrypt20, libgnutls30, libselinux1, libsemanage-common, libsemanage2 (+2 more, see the package list below) |
| rclone | bazil.org/fuse, cloud.google.com/go/auth, cloud.google.com/go/auth/oauth2adapt, cloud.google.com/go/compute/metadata, github.com/Azure/azure-sdk-for-go/sdk/azcore, github.com/Azure/azure-sdk-for-go/sdk/azidentity, github.com/Azure/azure-sdk-for-go/sdk/internal, github.com/Azure/azure-sdk-for-go/sdk/storage/azblob, github.com/Azure/azure-sdk-for-go/sdk/storage/azfile, github.com/Azure/go-ntlmssp, github.com/AzureAD/microsoft-authentication-library-for-go, github.com/FilenCloudDienste/filen-sdk-go (+231 more, see the package list below) |

Copyleft and weak-copyleft licenses appear on operating-system packages inside the images (not in the Rust executables). They apply to those packages, which are distributed in the images unmodified:

| Image | License identifiers on OS packages (package counts) |
| --- | --- |
| inside-agent | GPL-2.0-only (57), GPL-2.0-or-later (50), GPL-3.0-only (29), GPL-3.0-or-later (28), LGPL-2.1-only (24), LGPL-2.0-only (22), LGPL-2.0-or-later (18), LGPL-2.1-or-later (18) |
| outside-agent | GPL-2.0-only (57), GPL-2.0-or-later (50), GPL-3.0-only (29), GPL-3.0-or-later (28), LGPL-2.1-only (24), LGPL-2.0-only (22), LGPL-2.0-or-later (18), LGPL-2.1-or-later (18) |
| rclone | GPL-2.0-only (8), GPL-2.0-only AND LGPL-2.1-only (3), MPL-2.0 AND MIT (2), MIT AND BSD-2-Clause AND GPL-2.0-or-later (1) |

## Redistribution notes

- Most linked crates are MIT, Apache-2.0 or BSD-style. Their terms require keeping copyright and license notices when you redistribute the binaries or images. The published images do not currently bundle those notice texts; the *Evidence* column shows where they live in each crate's source. Add them to the image or distribute them alongside it if you publish builds.
- The Mozilla CA bundle (`webpki-roots` or the OS `ca-certificates` package) carries its own license terms (see the callout table and the image inventory).
- This project does not bundle the Datum fonts or artwork; the dashboard only borrows a color palette and type feel from datum.net and falls back to system fonts. Datum's name and branding are not licensed by The Unlicense.
- Runtime services used over the network (the LLM API, an IP-geolocation lookup, Datum's Iroh relays, and the hosting platforms) have their own terms and are not software redistributed here.
- Postgres (`docker.io/library/postgres`) is pulled unmodified at deploy time under the PostgreSQL License plus its Debian packages' licenses; it is not built or redistributed by this repository. The rclone image extends the upstream `rclone/rclone` image (MIT) with the demo files in `demo/rclone-data`.

## Rust crates linked into the executables

| Crate | Version | Relationship | Used by | License | Evidence |
| --- | --- | --- | --- | --- | --- |
| [aead](https://crates.io/crates/aead/0.5.2) | 0.5.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [aes](https://crates.io/crates/aes/0.8.4) | 0.8.4 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [aes-gcm](https://crates.io/crates/aes-gcm/0.10.3) | 0.10.3 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [anyhow](https://crates.io/crates/anyhow/1.0.104) | 1.0.104 | Direct | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [arc-swap](https://crates.io/crates/arc-swap/1.9.3) | 1.9.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [arrayvec](https://crates.io/crates/arrayvec/0.7.8) | 0.7.8 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [async-trait](https://crates.io/crates/async-trait/0.1.92) | 0.1.92 | Transitive (proc-macro) | inside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [atomic-waker](https://crates.io/crates/atomic-waker/1.1.2) | 1.1.2 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT; LICENSE-THIRD-PARTY |
| [attohttpc](https://crates.io/crates/attohttpc/0.30.1) | 0.30.1 | Transitive | inside-agent, outside-agent | MPL-2.0 | LICENSE |
| [axum](https://crates.io/crates/axum/0.8.9) | 0.8.9 | Direct | inside-agent, outside-agent | MIT | LICENSE |
| [axum-core](https://crates.io/crates/axum-core/0.5.6) | 0.5.6 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [backon](https://crates.io/crates/backon/1.6.0) | 1.6.0 | Transitive | inside-agent, outside-agent | Apache-2.0 | LICENSE |
| [base16ct](https://crates.io/crates/base16ct/1.0.0) | 1.0.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [base64](https://crates.io/crates/base64/0.22.1) | 0.22.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [base64](https://crates.io/crates/base64/0.23.1) | 0.23.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [base64ct](https://crates.io/crates/base64ct/1.8.3) | 1.8.3 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [bitflags](https://crates.io/crates/bitflags/2.13.2) | 2.13.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [blake3](https://crates.io/crates/blake3/1.8.7) | 1.8.7 | Transitive | inside-agent, outside-agent | CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception | LICENSE_A2; LICENSE_A2LLVM; LICENSE_CC0 |
| [block-buffer](https://crates.io/crates/block-buffer/0.12.1) | 0.12.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [byteorder](https://crates.io/crates/byteorder/1.5.0) | 1.5.0 | Transitive | inside-agent, outside-agent | Unlicense OR MIT | COPYING; LICENSE-MIT; UNLICENSE |
| [bytes](https://crates.io/crates/bytes/1.12.1) | 1.12.1 | Direct | inside-agent, outside-agent | MIT | LICENSE |
| [cfg-if](https://crates.io/crates/cfg-if/1.0.5) | 1.0.5 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [chacha20](https://crates.io/crates/chacha20/0.10.2) | 0.10.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [cipher](https://crates.io/crates/cipher/0.4.4) | 0.4.4 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [cmov](https://crates.io/crates/cmov/0.5.4) | 0.5.4 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [cobs](https://crates.io/crates/cobs/0.3.0) | 0.3.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [const-oid](https://crates.io/crates/const-oid/0.10.2) | 0.10.2 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [constant_time_eq](https://crates.io/crates/constant_time_eq/0.4.2) | 0.4.2 | Transitive | inside-agent, outside-agent | CC0-1.0 OR MIT-0 OR Apache-2.0 | LICENSE-APACHE; LICENSE-CC0; LICENSE-MIT0 |
| [convert_case](https://crates.io/crates/convert_case/0.10.0) | 0.10.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [cordyceps](https://crates.io/crates/cordyceps/0.3.5) | 0.3.5 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [cpufeatures](https://crates.io/crates/cpufeatures/0.2.17) | 0.2.17 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [cpufeatures](https://crates.io/crates/cpufeatures/0.3.1) | 0.3.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [crypto-common](https://crates.io/crates/crypto-common/0.1.7) | 0.1.7 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [crypto-common](https://crates.io/crates/crypto-common/0.2.2) | 0.2.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [ctr](https://crates.io/crates/ctr/0.9.2) | 0.9.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [ctutils](https://crates.io/crates/ctutils/0.4.3) | 0.4.3 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [curve25519-dalek](https://crates.io/crates/curve25519-dalek/5.0.0) | 5.0.0 | Transitive | inside-agent, outside-agent | BSD-3-Clause | LICENSE |
| [curve25519-dalek-derive](https://crates.io/crates/curve25519-dalek-derive/0.1.1) | 0.1.1 | Transitive (proc-macro) | inside-agent, outside-agent | MIT/Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [data-encoding](https://crates.io/crates/data-encoding/2.11.1) | 2.11.1 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [data-encoding-macro](https://crates.io/crates/data-encoding-macro/0.1.21) | 0.1.21 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [data-encoding-macro-internal](https://crates.io/crates/data-encoding-macro-internal/0.1.19) | 0.1.19 | Transitive (proc-macro) | inside-agent, outside-agent | MIT | LICENSE |
| [der](https://crates.io/crates/der/0.8.2) | 0.8.2 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [deranged](https://crates.io/crates/deranged/0.5.9) | 0.5.9 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-Apache; LICENSE-MIT |
| [derive_more](https://crates.io/crates/derive_more/2.1.1) | 2.1.1 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [derive_more-impl](https://crates.io/crates/derive_more-impl/2.1.1) | 2.1.1 | Transitive (proc-macro) | inside-agent, outside-agent | MIT | LICENSE |
| [diatomic-waker](https://crates.io/crates/diatomic-waker/0.2.3) | 0.2.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [digest](https://crates.io/crates/digest/0.11.3) | 0.11.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [displaydoc](https://crates.io/crates/displaydoc/0.2.7) | 0.2.7 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [ed25519](https://crates.io/crates/ed25519/3.0.0) | 3.0.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [ed25519-dalek](https://crates.io/crates/ed25519-dalek/3.0.0) | 3.0.0 | Transitive | inside-agent, outside-agent | BSD-3-Clause | LICENSE |
| [enum-assoc](https://crates.io/crates/enum-assoc/1.4.1) | 1.4.1 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | none found |
| [equivalent](https://crates.io/crates/equivalent/1.0.3) | 1.0.3 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [errno](https://crates.io/crates/errno/0.3.14) | 0.3.14 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [fallible-iterator](https://crates.io/crates/fallible-iterator/0.2.0) | 0.2.0 | Transitive | inside-agent | MIT/Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [fastrand](https://crates.io/crates/fastrand/2.5.0) | 2.5.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [fnv](https://crates.io/crates/fnv/1.0.7) | 1.0.7 | Transitive | inside-agent, outside-agent | Apache-2.0 / MIT | LICENSE-APACHE; LICENSE-MIT |
| [foldhash](https://crates.io/crates/foldhash/0.2.0) | 0.2.0 | Transitive | inside-agent, outside-agent | Zlib | LICENSE |
| [form_urlencoded](https://crates.io/crates/form_urlencoded/1.2.2) | 1.2.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures](https://crates.io/crates/futures/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-buffered](https://crates.io/crates/futures-buffered/0.2.13) | 0.2.13 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [futures-channel](https://crates.io/crates/futures-channel/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-core](https://crates.io/crates/futures-core/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-executor](https://crates.io/crates/futures-executor/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-io](https://crates.io/crates/futures-io/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-lite](https://crates.io/crates/futures-lite/2.6.1) | 2.6.1 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT; LICENSE-THIRD-PARTY |
| [futures-macro](https://crates.io/crates/futures-macro/0.3.34) | 0.3.34 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-sink](https://crates.io/crates/futures-sink/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-task](https://crates.io/crates/futures-task/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [futures-util](https://crates.io/crates/futures-util/0.3.34) | 0.3.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [generic-array](https://crates.io/crates/generic-array/0.14.7) | 0.14.7 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [getrandom](https://crates.io/crates/getrandom/0.2.17) | 0.2.17 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [getrandom](https://crates.io/crates/getrandom/0.4.3) | 0.4.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [ghash](https://crates.io/crates/ghash/0.5.1) | 0.5.1 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [h2](https://crates.io/crates/h2/0.4.20) | 0.4.20 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [hash32](https://crates.io/crates/hash32/0.2.1) | 0.2.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [hashbrown](https://crates.io/crates/hashbrown/0.17.1) | 0.17.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [heapless](https://crates.io/crates/heapless/0.7.17) | 0.7.17 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [heck](https://crates.io/crates/heck/0.5.0) | 0.5.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [hmac](https://crates.io/crates/hmac/0.13.0) | 0.13.0 | Transitive | inside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [http](https://crates.io/crates/http/1.5.0) | 1.5.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [http-body](https://crates.io/crates/http-body/1.1.0) | 1.1.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [http-body-util](https://crates.io/crates/http-body-util/0.1.5) | 0.1.5 | Direct | inside-agent, outside-agent | MIT | LICENSE |
| [httparse](https://crates.io/crates/httparse/1.10.1) | 1.10.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [httpdate](https://crates.io/crates/httpdate/1.0.3) | 1.0.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [hybrid-array](https://crates.io/crates/hybrid-array/0.4.15) | 0.4.15 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [hyper](https://crates.io/crates/hyper/1.12.0) | 1.12.0 | Direct | inside-agent, outside-agent | MIT | LICENSE |
| [hyper-rustls](https://crates.io/crates/hyper-rustls/0.27.10) | 0.27.10 | Transitive | inside-agent, outside-agent | Apache-2.0 OR ISC OR MIT | LICENSE-APACHE; LICENSE-ISC; LICENSE-MIT |
| [hyper-util](https://crates.io/crates/hyper-util/0.1.21) | 0.1.21 | Direct | inside-agent, outside-agent | MIT | LICENSE |
| [icu_collections](https://crates.io/crates/icu_collections/2.3.0) | 2.3.0 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [icu_locale_core](https://crates.io/crates/icu_locale_core/2.3.0) | 2.3.0 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [icu_normalizer](https://crates.io/crates/icu_normalizer/2.3.0) | 2.3.0 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [icu_normalizer_data](https://crates.io/crates/icu_normalizer_data/2.3.0) | 2.3.0 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [icu_properties](https://crates.io/crates/icu_properties/2.3.0) | 2.3.0 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [icu_properties_data](https://crates.io/crates/icu_properties_data/2.3.0) | 2.3.0 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [icu_provider](https://crates.io/crates/icu_provider/2.3.1) | 2.3.1 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [identity-hash](https://crates.io/crates/identity-hash/0.1.0) | 0.1.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [idna](https://crates.io/crates/idna/1.1.0) | 1.1.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [idna_adapter](https://crates.io/crates/idna_adapter/1.2.2) | 1.2.2 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [if-addrs](https://crates.io/crates/if-addrs/0.13.4) | 0.13.4 | Direct | outside-agent | MIT OR BSD-3-Clause | LICENSE-BSD; LICENSE-MIT |
| [igd-next](https://crates.io/crates/igd-next/0.17.1) | 0.17.1 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [indexmap](https://crates.io/crates/indexmap/2.14.2) | 2.14.2 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [inout](https://crates.io/crates/inout/0.1.4) | 0.1.4 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [ipnet](https://crates.io/crates/ipnet/2.12.2) | 2.12.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [iroh](https://crates.io/crates/iroh/1.4.0) | 1.4.0 | Direct | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-BSD3 |
| [iroh-base](https://crates.io/crates/iroh-base/1.4.0) | 1.4.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | none found |
| [iroh-dns](https://crates.io/crates/iroh-dns/1.4.0) | 1.4.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | none found |
| [iroh-metrics](https://crates.io/crates/iroh-metrics/1.0.2) | 1.0.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [iroh-metrics-derive](https://crates.io/crates/iroh-metrics-derive/1.0.1) | 1.0.1 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | none found |
| [iroh-relay](https://crates.io/crates/iroh-relay/1.4.0) | 1.4.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-BSD3 |
| [iroh-tickets](https://crates.io/crates/iroh-tickets/1.0.0) | 1.0.0 | Direct | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [itoa](https://crates.io/crates/itoa/1.0.18) | 1.0.18 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [libc](https://crates.io/crates/libc/0.2.191) | 0.2.191 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [litemap](https://crates.io/crates/litemap/0.8.3) | 0.8.3 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [lock_api](https://crates.io/crates/lock_api/0.4.14) | 0.4.14 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [log](https://crates.io/crates/log/0.4.34) | 0.4.34 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [lru](https://crates.io/crates/lru/0.18.5) | 0.18.5 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [lru-slab](https://crates.io/crates/lru-slab/0.1.3) | 0.1.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 OR Zlib | LICENSE-APACHE; LICENSE-MIT; LICENSE-ZLIB |
| [mac-addr](https://crates.io/crates/mac-addr/0.3.0) | 0.3.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [matchit](https://crates.io/crates/matchit/0.8.4) | 0.8.4 | Transitive | inside-agent, outside-agent | MIT AND BSD-3-Clause | LICENSE; LICENSE.httprouter |
| [md-5](https://crates.io/crates/md-5/0.11.0) | 0.11.0 | Transitive | inside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [memchr](https://crates.io/crates/memchr/2.8.3) | 2.8.3 | Transitive | inside-agent, outside-agent | Unlicense OR MIT | COPYING; LICENSE-MIT; UNLICENSE |
| [mime](https://crates.io/crates/mime/0.3.17) | 0.3.17 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [mio](https://crates.io/crates/mio/1.2.4) | 1.2.4 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [n0-dns-resolver](https://crates.io/crates/n0-dns-resolver/0.1.0) | 0.1.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [n0-error](https://crates.io/crates/n0-error/1.0.1) | 1.0.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [n0-error-macros](https://crates.io/crates/n0-error-macros/1.0.1) | 1.0.1 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | none found |
| [n0-future](https://crates.io/crates/n0-future/0.3.2) | 0.3.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [n0-watcher](https://crates.io/crates/n0-watcher/1.0.1) | 1.0.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [netdev](https://crates.io/crates/netdev/0.46.3) | 0.46.3 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [netlink-packet-core](https://crates.io/crates/netlink-packet-core/0.8.2) | 0.8.2 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [netlink-packet-core](https://crates.io/crates/netlink-packet-core/0.9.0) | 0.9.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [netlink-packet-route](https://crates.io/crates/netlink-packet-route/0.31.0) | 0.31.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [netlink-packet-route](https://crates.io/crates/netlink-packet-route/0.33.0) | 0.33.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [netlink-proto](https://crates.io/crates/netlink-proto/0.12.2) | 0.12.2 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [netlink-sys](https://crates.io/crates/netlink-sys/0.8.8) | 0.8.8 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [netlink-sys](https://crates.io/crates/netlink-sys/0.9.0) | 0.9.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [netwatch](https://crates.io/crates/netwatch/0.19.3) | 0.19.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | none found |
| [noq](https://crates.io/crates/noq/1.4.0) | 1.4.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [noq-proto](https://crates.io/crates/noq-proto/1.4.0) | 1.4.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [noq-udp](https://crates.io/crates/noq-udp/1.4.0) | 1.4.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [num-conv](https://crates.io/crates/num-conv/0.2.2) | 0.2.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-Apache; LICENSE-MIT |
| [num_enum](https://crates.io/crates/num_enum/0.7.6) | 0.7.6 | Transitive | inside-agent, outside-agent | BSD-3-Clause OR MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-BSD; LICENSE-MIT |
| [num_enum_derive](https://crates.io/crates/num_enum_derive/0.7.6) | 0.7.6 | Transitive (proc-macro) | inside-agent, outside-agent | BSD-3-Clause OR MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-BSD; LICENSE-MIT |
| [once_cell](https://crates.io/crates/once_cell/1.21.4) | 1.21.4 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [opaque-debug](https://crates.io/crates/opaque-debug/0.3.1) | 0.3.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [openssl-probe](https://crates.io/crates/openssl-probe/0.2.1) | 0.2.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [papaya](https://crates.io/crates/papaya/0.2.5) | 0.2.5 | Transitive | inside-agent, outside-agent | MIT | LICENSE.md |
| [parking](https://crates.io/crates/parking/2.2.1) | 2.2.1 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT; LICENSE-THIRD-PARTY |
| [parking_lot](https://crates.io/crates/parking_lot/0.12.5) | 0.12.5 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [parking_lot_core](https://crates.io/crates/parking_lot_core/0.9.12) | 0.9.12 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [paste](https://crates.io/crates/paste/1.0.15) | 1.0.15 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [pem-rfc7468](https://crates.io/crates/pem-rfc7468/1.0.0) | 1.0.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [percent-encoding](https://crates.io/crates/percent-encoding/2.3.2) | 2.3.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [phf](https://crates.io/crates/phf/0.13.1) | 0.13.1 | Transitive | inside-agent | MIT | LICENSE |
| [phf_shared](https://crates.io/crates/phf_shared/0.13.1) | 0.13.1 | Transitive | inside-agent | MIT | LICENSE |
| [pin-project](https://crates.io/crates/pin-project/1.1.13) | 1.1.13 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [pin-project-internal](https://crates.io/crates/pin-project-internal/1.1.13) | 1.1.13 | Transitive (proc-macro) | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [pin-project-lite](https://crates.io/crates/pin-project-lite/0.2.17) | 0.2.17 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [pkcs8](https://crates.io/crates/pkcs8/0.11.0) | 0.11.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [polyval](https://crates.io/crates/polyval/0.6.2) | 0.6.2 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [portable-atomic](https://crates.io/crates/portable-atomic/1.15.0) | 1.15.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [portmapper](https://crates.io/crates/portmapper/0.19.3) | 0.19.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | none found |
| [postcard](https://crates.io/crates/postcard/1.1.3) | 1.1.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [postcard-derive](https://crates.io/crates/postcard-derive/0.2.2) | 0.2.2 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [postgres-protocol](https://crates.io/crates/postgres-protocol/0.6.12) | 0.6.12 | Transitive | inside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [postgres-types](https://crates.io/crates/postgres-types/0.2.14) | 0.2.14 | Transitive | inside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [potential_utf](https://crates.io/crates/potential_utf/0.1.6) | 0.1.6 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [powerfmt](https://crates.io/crates/powerfmt/0.2.1) | 0.2.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-Apache; LICENSE-MIT |
| [proc-macro-crate](https://crates.io/crates/proc-macro-crate/3.5.0) | 3.5.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [proc-macro2](https://crates.io/crates/proc-macro2/1.0.107) | 1.0.107 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [quote](https://crates.io/crates/quote/1.0.47) | 1.0.47 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [rand](https://crates.io/crates/rand/0.10.3) | 0.10.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [rand_core](https://crates.io/crates/rand_core/0.10.1) | 0.10.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [rand_pcg](https://crates.io/crates/rand_pcg/0.10.2) | 0.10.2 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [reqwest](https://crates.io/crates/reqwest/0.12.28) | 0.12.28 | Direct | inside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [reqwest](https://crates.io/crates/reqwest/0.13.5) | 0.13.5 | Direct | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [ring](https://crates.io/crates/ring/0.17.14) | 0.17.14 | Transitive | inside-agent, outside-agent | Apache-2.0 AND ISC | LICENSE; LICENSE-BoringSSL; LICENSE-other-bits |
| [rustc-hash](https://crates.io/crates/rustc-hash/2.1.3) | 2.1.3 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [rustls](https://crates.io/crates/rustls/0.23.46) | 0.23.46 | Transitive | inside-agent, outside-agent | Apache-2.0 OR ISC OR MIT | LICENSE-APACHE; LICENSE-ISC; LICENSE-MIT |
| [rustls-native-certs](https://crates.io/crates/rustls-native-certs/0.8.5) | 0.8.5 | Transitive | inside-agent, outside-agent | Apache-2.0 OR ISC OR MIT | LICENSE; LICENSE-APACHE; LICENSE-ISC; LICENSE-MIT |
| [rustls-pki-types](https://crates.io/crates/rustls-pki-types/1.15.1) | 1.15.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [rustls-platform-verifier](https://crates.io/crates/rustls-platform-verifier/0.7.1) | 0.7.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [rustls-webpki](https://crates.io/crates/rustls-webpki/0.103.16) | 0.103.16 | Transitive | inside-agent, outside-agent | ISC | LICENSE |
| [rustversion](https://crates.io/crates/rustversion/1.0.23) | 1.0.23 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [ryu](https://crates.io/crates/ryu/1.0.23) | 1.0.23 | Transitive | inside-agent, outside-agent | Apache-2.0 OR BSL-1.0 | LICENSE-APACHE; LICENSE-BOOST |
| [scopeguard](https://crates.io/crates/scopeguard/1.2.0) | 1.2.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [seize](https://crates.io/crates/seize/0.5.1) | 0.5.1 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [serde](https://crates.io/crates/serde/1.0.229) | 1.0.229 | Direct | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [serde_bytes](https://crates.io/crates/serde_bytes/0.11.19) | 0.11.19 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [serde_core](https://crates.io/crates/serde_core/1.0.229) | 1.0.229 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [serde_derive](https://crates.io/crates/serde_derive/1.0.229) | 1.0.229 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [serde_json](https://crates.io/crates/serde_json/1.0.154) | 1.0.154 | Direct | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [serde_path_to_error](https://crates.io/crates/serde_path_to_error/0.1.20) | 0.1.20 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [serde_urlencoded](https://crates.io/crates/serde_urlencoded/0.7.1) | 0.7.1 | Transitive | inside-agent, outside-agent | MIT/Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [serdect](https://crates.io/crates/serdect/0.4.3) | 0.4.3 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [sha1_smol](https://crates.io/crates/sha1_smol/1.0.1) | 1.0.1 | Transitive | inside-agent, outside-agent | BSD-3-Clause | LICENSE |
| [sha2](https://crates.io/crates/sha2/0.11.1) | 0.11.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [signal-hook-registry](https://crates.io/crates/signal-hook-registry/1.4.8) | 1.4.8 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [signature](https://crates.io/crates/signature/3.0.0) | 3.0.0 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [simdutf8](https://crates.io/crates/simdutf8/0.1.5) | 0.1.5 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-Apache; LICENSE-MIT |
| [simple-dns](https://crates.io/crates/simple-dns/0.12.0) | 0.12.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [siphasher](https://crates.io/crates/siphasher/1.0.4) | 1.0.4 | Transitive | inside-agent | MIT OR Apache-2.0 | COPYING; LICENSE-APACHE; LICENSE-MIT |
| [slab](https://crates.io/crates/slab/0.4.12) | 0.4.12 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [slotmap](https://crates.io/crates/slotmap/1.1.1) | 1.1.1 | Transitive | inside-agent, outside-agent | Zlib | LICENSE |
| [smallvec](https://crates.io/crates/smallvec/1.16.3) | 1.16.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [socket2](https://crates.io/crates/socket2/0.6.5) | 0.6.5 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [sorted-index-buffer](https://crates.io/crates/sorted-index-buffer/0.2.1) | 0.2.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [spez](https://crates.io/crates/spez/0.1.2) | 0.1.2 | Transitive (proc-macro) | inside-agent, outside-agent | BSD-2-Clause | LICENSE |
| [spin](https://crates.io/crates/spin/0.10.1) | 0.10.1 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [spin](https://crates.io/crates/spin/0.9.9) | 0.9.9 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [spki](https://crates.io/crates/spki/0.8.1) | 0.8.1 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [stable_deref_trait](https://crates.io/crates/stable_deref_trait/1.2.1) | 1.2.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [stringprep](https://crates.io/crates/stringprep/0.1.5) | 0.1.5 | Transitive | inside-agent | MIT/Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [strum](https://crates.io/crates/strum/0.28.0) | 0.28.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [strum_macros](https://crates.io/crates/strum_macros/0.28.0) | 0.28.0 | Transitive (proc-macro) | inside-agent, outside-agent | MIT | LICENSE |
| [subtle](https://crates.io/crates/subtle/2.6.1) | 2.6.1 | Transitive | inside-agent, outside-agent | BSD-3-Clause | LICENSE |
| [syn](https://crates.io/crates/syn/2.0.119) | 2.0.119 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [syn](https://crates.io/crates/syn/3.0.9) | 3.0.9 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [sync_wrapper](https://crates.io/crates/sync_wrapper/1.0.2) | 1.0.2 | Transitive | inside-agent, outside-agent | Apache-2.0 | LICENSE |
| [synstructure](https://crates.io/crates/synstructure/0.14.0) | 0.14.0 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [thiserror](https://crates.io/crates/thiserror/2.0.21) | 2.0.21 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [thiserror-impl](https://crates.io/crates/thiserror-impl/2.0.21) | 2.0.21 | Transitive (proc-macro) | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [time](https://crates.io/crates/time/0.3.55) | 0.3.55 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-Apache; LICENSE-MIT |
| [time-core](https://crates.io/crates/time-core/0.1.9) | 0.1.9 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-Apache; LICENSE-MIT |
| [tinystr](https://crates.io/crates/tinystr/0.8.4) | 0.8.4 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [tinyvec](https://crates.io/crates/tinyvec/1.13.3) | 1.13.3 | Transitive | inside-agent, outside-agent | Zlib OR Apache-2.0 OR MIT | LICENSE-APACHE.md; LICENSE-MIT.md; LICENSE-ZLIB.md |
| [tokio](https://crates.io/crates/tokio/1.54.0) | 1.54.0 | Direct | inside-agent, outside-agent | MIT | LICENSE |
| [tokio-macros](https://crates.io/crates/tokio-macros/2.7.2) | 2.7.2 | Transitive (proc-macro) | inside-agent, outside-agent | MIT | LICENSE |
| [tokio-postgres](https://crates.io/crates/tokio-postgres/0.7.18) | 0.7.18 | Direct | inside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [tokio-rustls](https://crates.io/crates/tokio-rustls/0.26.6) | 0.26.6 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [tokio-stream](https://crates.io/crates/tokio-stream/0.1.19) | 0.1.19 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [tokio-util](https://crates.io/crates/tokio-util/0.7.20) | 0.7.20 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [tokio-websockets](https://crates.io/crates/tokio-websockets/0.13.3) | 0.13.3 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [toml_datetime](https://crates.io/crates/toml_datetime/1.1.2+spec-1.1.0) | 1.1.2+spec-1.1.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [toml_edit](https://crates.io/crates/toml_edit/0.25.17+spec-1.1.0) | 0.25.17+spec-1.1.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [toml_parser](https://crates.io/crates/toml_parser/1.1.5+spec-1.1.0) | 1.1.5+spec-1.1.0 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [tower](https://crates.io/crates/tower/0.5.3) | 0.5.3 | Direct | inside-agent, outside-agent | MIT | LICENSE |
| [tower-http](https://crates.io/crates/tower-http/0.6.11) | 0.6.11 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [tower-layer](https://crates.io/crates/tower-layer/0.3.3) | 0.3.3 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [tower-service](https://crates.io/crates/tower-service/0.3.3) | 0.3.3 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [tracing](https://crates.io/crates/tracing/0.1.44) | 0.1.44 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [tracing-attributes](https://crates.io/crates/tracing-attributes/0.1.31) | 0.1.31 | Transitive (proc-macro) | inside-agent, outside-agent | MIT | LICENSE |
| [tracing-core](https://crates.io/crates/tracing-core/0.1.36) | 0.1.36 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [try-lock](https://crates.io/crates/try-lock/0.2.5) | 0.2.5 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [typenum](https://crates.io/crates/typenum/1.20.1) | 1.20.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE; LICENSE-APACHE; LICENSE-MIT |
| [unicode-bidi](https://crates.io/crates/unicode-bidi/0.3.18) | 0.3.18 | Transitive | inside-agent | MIT OR Apache-2.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [unicode-ident](https://crates.io/crates/unicode-ident/1.0.27) | 1.0.27 | Transitive | inside-agent, outside-agent | (MIT OR Apache-2.0) AND Unicode-3.0 | LICENSE-APACHE; LICENSE-MIT; LICENSE-UNICODE |
| [unicode-normalization](https://crates.io/crates/unicode-normalization/0.1.26) | 0.1.26 | Transitive | inside-agent | (MIT OR Apache-2.0) AND Unicode-3.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT; LICENSE-UNICODE |
| [unicode-properties](https://crates.io/crates/unicode-properties/0.1.4) | 0.1.4 | Transitive | inside-agent | MIT/Apache-2.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [unicode-segmentation](https://crates.io/crates/unicode-segmentation/1.13.3) | 1.13.3 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [unicode-xid](https://crates.io/crates/unicode-xid/0.2.6) | 0.2.6 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [universal-hash](https://crates.io/crates/universal-hash/0.5.1) | 0.5.1 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [untrusted](https://crates.io/crates/untrusted/0.9.0) | 0.9.0 | Transitive | inside-agent, outside-agent | ISC | LICENSE.txt |
| [url](https://crates.io/crates/url/2.5.8) | 2.5.8 | Transitive | inside-agent, outside-agent | MIT OR Apache-2.0 | LICENSE-APACHE; LICENSE-MIT |
| [utf8_iter](https://crates.io/crates/utf8_iter/1.0.4) | 1.0.4 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | COPYRIGHT; LICENSE-APACHE; LICENSE-MIT |
| [want](https://crates.io/crates/want/0.3.2) | 0.3.2 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [webpki-roots](https://crates.io/crates/webpki-roots/1.0.10) | 1.0.10 | Transitive | inside-agent, outside-agent | CDLA-Permissive-2.0 | LICENSE |
| [whoami](https://crates.io/crates/whoami/2.1.3) | 2.1.3 | Transitive | inside-agent | Apache-2.0 OR BSL-1.0 OR MIT | LICENSE_APACHE; LICENSE_BOOST; LICENSE_MIT |
| [winnow](https://crates.io/crates/winnow/1.0.4) | 1.0.4 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |
| [writeable](https://crates.io/crates/writeable/0.6.4) | 0.6.4 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [xml-rs](https://crates.io/crates/xml-rs/0.8.29) | 0.8.29 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [xmltree](https://crates.io/crates/xmltree/0.10.3) | 0.10.3 | Transitive | inside-agent, outside-agent | MIT | LICENSE |
| [yoke](https://crates.io/crates/yoke/0.8.3) | 0.8.3 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [yoke-derive](https://crates.io/crates/yoke-derive/0.8.4) | 0.8.4 | Transitive (proc-macro) | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [zerocopy](https://crates.io/crates/zerocopy/0.8.63) | 0.8.63 | Transitive | inside-agent, outside-agent | BSD-2-Clause OR Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-BSD; LICENSE-MIT |
| [zerocopy-derive](https://crates.io/crates/zerocopy-derive/0.8.63) | 0.8.63 | Transitive (proc-macro) | inside-agent, outside-agent | BSD-2-Clause OR Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-BSD; LICENSE-MIT |
| [zerofrom](https://crates.io/crates/zerofrom/0.1.8) | 0.1.8 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [zerofrom-derive](https://crates.io/crates/zerofrom-derive/0.1.8) | 0.1.8 | Transitive (proc-macro) | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [zeroize](https://crates.io/crates/zeroize/1.9.1) | 1.9.1 | Transitive | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [zeroize_derive](https://crates.io/crates/zeroize_derive/1.5.0) | 1.5.0 | Transitive (proc-macro) | inside-agent, outside-agent | Apache-2.0 OR MIT | LICENSE-APACHE; LICENSE-MIT |
| [zerotrie](https://crates.io/crates/zerotrie/0.2.5) | 0.2.5 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [zerovec](https://crates.io/crates/zerovec/0.11.8) | 0.11.8 | Transitive | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [zerovec-derive](https://crates.io/crates/zerovec-derive/0.11.6) | 0.11.6 | Transitive (proc-macro) | inside-agent, outside-agent | Unicode-3.0 | LICENSE |
| [zmij](https://crates.io/crates/zmij/1.0.23) | 1.0.23 | Transitive | inside-agent, outside-agent | MIT | LICENSE-MIT |

## Container images

Each image is a Debian or Alpine base plus one Rust executable (dynamically linked against glibc) or the rclone binary. Executables built by Rust contain the crates above; Syft cannot list them from a stripped binary.

### inside-agent

Image ID `cc75f9b6a9667daf5040aa594a911ec300094ea69d6f4f8a8da3f6b398340e4d`; base `Debian GNU/Linux 12 (bookworm)`; **91** packages detected (91 deb).

<details>
<summary>Package list</summary>

| Package | Version | Type | License identifiers found |
| --- | --- | --- | --- |
| adduser | 3.134 | deb | GPL-2.0-only; GPL-2.0-or-later |
| apt | 2.6.1 | deb | BSD-3-Clause; Expat; GPL-2.0-only; GPL-2.0-or-later |
| base-files | 12.4+deb12u15 | deb | NOASSERTION |
| base-passwd | 3.6.1 | deb | GPL-2.0-only; public-domain |
| bash | 5.2.15-2+b13 | deb | BSD-4-Clause-UC; GFDL-1.3-only; GFDL-NIV-1.3; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; Latex2e; MIT-like; permissive |
| bsdutils | 1:2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| ca-certificates | 20250419~deb12u2 | deb | GPL-2.0-only; GPL-2.0-or-later; MPL-2.0 |
| coreutils | 9.1-1 | deb | BSD-4-Clause-UC; FSFULLR; GFDL-1.3-only; GFDL-NIV-1.3; GPL-3.0-only; GPL-3.0-or-later; ISC |
| dash | 0.5.12-2 | deb | BSD-3-Clause; GPL-2.0-only; GPL-2.0-or-later; public-domain |
| debconf | 1.5.82 | deb | BSD-2-Clause |
| debian-archive-keyring | 2023.3+deb12u2 | deb | NOASSERTION |
| debianutils | 5.7-0.5~deb12u1 | deb | GPL-2.0-only; GPL-2.0-or-later; SMAIL-GPL; public-domain |
| diffutils | 1:3.8-4 | deb | FSFAP; FSFULLR; GFDL-1.3-only; GFDL-NIV-1.3; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; X11; public-domain |
| dpkg | 1.21.23 | deb | BSD-2-Clause; GPL-2.0-only; GPL-2.0-or-later; public-domain-s-s-d |
| e2fsprogs | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| findutils | 4.9.0-4 | deb | BSD-3-Clause; FSFAP; FSFULLR; GFDL-1.3-only; GFDL-NIV-1.3+; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; ISC; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; X11; public-domain |
| gcc-12-base | 12.2.0-14+deb12u1 | deb | NOASSERTION |
| gpgv | 2.2.40-1.1+deb12u2 | deb | BSD-3-Clause; CC0-1.0; Expat; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; RFC-Reference; TinySCHEME; permissive |
| grep | 3.8-5 | deb | GPL-3.0-only; GPL-3.0-or-later |
| gzip | 1.12-1 | deb | FSF-manpages; GFDL-1.3+-no-invariant; GFDL-3; GPL-3.0-only; GPL-3.0-or-later |
| hostname | 3.23+nmu1 | deb | GPL-2.0-only |
| init-system-helpers | 1.65.2+deb12u1 | deb | BSD-3-Clause; GPL-2.0-only; GPL-2.0-or-later |
| libacl1 | 2.3.1-3 | deb | GPL-2.0-only; GPL-2.0-or-later; LGPL-2.0-or-later; LGPL-2.1-only |
| libapt-pkg6.0 | 2.6.1 | deb | BSD-3-Clause; Expat; GPL-2.0-only; GPL-2.0-or-later |
| libattr1 | 1:2.5.1-4 | deb | GPL-2.0-only; GPL-2.0-or-later; LGPL-2.0-or-later; LGPL-2.1-only |
| libaudit-common | 1:3.0.9-1 | deb | GPL-1.0-only; GPL-2.0-only; LGPL-2.1-only |
| libaudit1 | 1:3.0.9-1 | deb | GPL-1.0-only; GPL-2.0-only; LGPL-2.1-only |
| libblkid1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libbz2-1.0 | 1.0.8-5+b1 | deb | BSD-variant; GPL-2.0-only |
| libc-bin | 2.36-9+deb12u14 | deb | NOASSERTION |
| libc6 | 2.36-9+deb12u14 | deb | NOASSERTION |
| libcap-ng0 | 0.8.3-1+b3 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; LGPL-2.1-only; LGPL-2.1-or-later |
| libcap2 | 1:2.66-4+deb12u3+b1 | deb | BSD-3-Clause; GPL-2.0-only; GPL-2.0-or-later |
| libcom-err2 | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| libcrypt1 | 1:4.4.33-2 | deb | NOASSERTION |
| libdb5.3 | 5.3.28+dfsg2-1 | deb | Artistic; BSD-3-Clause; BSD-3-clause-fjord; GPL; GPL-3.0-only; MIT-old; MS-PL; Sleepycat; TCL-like; X11; Zlib |
| libdebconfclient0 | 0.270 | deb | BSD-2-Clause; GPL-2.0-only; GPL-2.0-or-later |
| libext2fs2 | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| libffi8 | 3.4.4-1 | deb | Expat; GPL; GPL-2.0-or-later; GPL-3.0-or-later; MPL-1.1; X11; public-domain |
| libgcc-s1 | 12.2.0-14+deb12u1 | deb | NOASSERTION |
| libgcrypt20 | 1.10.1-3+deb12u1 | deb | NOASSERTION |
| libgmp10 | 2:6.2.1+dfsg1-1.1 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later |
| libgnutls30 | 3.7.9-2+deb12u7 | deb | NOASSERTION |
| libgpg-error0 | 1.46-1 | deb | BSD-3-Clause; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; g10-permissive |
| libhogweed6 | 3.8.1-2 | deb | Expat; GAP; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-3.0-or-later; public-domain |
| libidn2-0 | 2.3.3-1+b1 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later; Unicode |
| liblz4-1 | 1.9.4-1 | deb | BSD-2-Clause; GPL-2.0-only; GPL-2.0-or-later |
| liblzma5 | 5.4.1-1+deb12u2 | deb | Autoconf; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; LGPL-2.0-only; LGPL-2.1-only; LGPL-2.1-or-later; PD; PD-debian; config-h; noderivs; permissive-fsf; permissive-nowarranty; probably-PD |
| libmd0 | 1.0.4-2 | deb | BSD-2-Clause; BSD-3-Clause; BSD-3-clause-Aaron-D-Gifford; Beerware; ISC; public-domain-md4; public-domain-md5; public-domain-sha1 |
| libmount1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libnettle8 | 3.8.1-2 | deb | Expat; GAP; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-3.0-or-later; public-domain |
| libp11-kit0 | 0.24.1-2 | deb | Apache-2.0; BSD-3-Clause; ISC; ISC+IBM; LGPL-2.1-only; LGPL-2.1-or-later; permissive-like-automake-output; same-as-rest-of-p11kit |
| libpam-modules | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpam-modules-bin | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpam-runtime | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpam0g | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpcre2-8-0 | 10.42-1+deb12u2 | deb | BSD-2-Clause; BSD-3-Clause; BSD-3-clause-Cambridge; X11; public-domain |
| libseccomp2 | 2.5.4-1+deb12u1 | deb | LGPL-2.1-only |
| libselinux1 | 3.4-1+b6 | deb | NOASSERTION |
| libsemanage-common | 3.4-1 | deb | NOASSERTION |
| libsemanage2 | 3.4-1+b5 | deb | NOASSERTION |
| libsepol2 | 3.4-2.1 | deb | GPL-2.0-only; GPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; Zlib |
| libsmartcols1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libss2 | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| libssl3 | 3.0.22-1~deb12u1 | deb | Apache-2.0; Artistic; GPL-1.0-only; GPL-1.0-or-later |
| libstdc++6 | 12.2.0-14+deb12u1 | deb | NOASSERTION |
| libsystemd0 | 252.39-1~deb12u2 | deb | CC0-1.0; Expat; GPL-2.0-only; GPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; public-domain |
| libtasn1-6 | 4.19.0-2+deb12u1 | deb | NOASSERTION |
| libtinfo6 | 6.4-4 | deb | BSD-3-Clause; MIT/X11; X11 |
| libudev1 | 252.39-1~deb12u2 | deb | CC0-1.0; Expat; GPL-2.0-only; GPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; public-domain |
| libunistring2 | 1.0-2 | deb | FreeSoftware; GFDL-1.2-only; GFDL-1.2-or-later; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT |
| libuuid1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libxxhash0 | 0.8.1-1 | deb | BSD-2-Clause; GPL-2.0-only |
| libzstd1 | 1.5.4+dfsg2-5 | deb | BSD-3-Clause; Expat; GPL-2.0-only; Zlib |
| login | 1:4.13+dfsg1-1+deb12u2 | deb | BSD-3-Clause; GPL-1.0-only; GPL-2.0-only; GPL-2.0-or-later; public-domain |
| logsave | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| mawk | 1.3.4.20200120-3.1 | deb | CC-BY-3.0; GPL-2.0-only; X11 |
| mount | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| ncurses-base | 6.4-4 | deb | BSD-3-Clause; MIT/X11; X11 |
| ncurses-bin | 6.4-4 | deb | BSD-3-Clause; MIT/X11; X11 |
| openssl | 3.0.22-1~deb12u1 | deb | Apache-2.0; Artistic; GPL-1.0-only; GPL-1.0-or-later |
| passwd | 1:4.13+dfsg1-1+deb12u2 | deb | BSD-3-Clause; GPL-1.0-only; GPL-2.0-only; GPL-2.0-or-later; public-domain |
| perl-base | 5.36.0-7+deb12u4 | deb | Artistic; Artistic-2.0; Artistic-dist; BSD-3-Clause; BSD-3-clause-GENERIC; BSD-3-clause-with-weird-numbering; BSD-4-clause-POWERDOG; BZIP; DONT-CHANGE-THE-GPL; Expat; GPL-1.0-only; GPL-1.0-or-later; GPL-2.0-only; GPL-2.0-or-later; GPL-3+-WITH-BISON-EXCEPTION; HSIEH-BSD; HSIEH-DERIVATIVE; LGPL-2.1-only; REGCOMP; REGCOMP,; RRA-KEEP-THIS-NOTICE; SDBM-PUBLIC-DOMAIN; TEXT-TABS; Unicode; Zlib |
| sed | 4.9-1+deb12u1 | deb | BSD-4-Clause-UC; BSL-1.0; GFDL-1.3-only; GFDL-NIV-1.3+; GPL-3.0-only; GPL-3.0-or-later; ISC; X11; pcre |
| sysvinit-utils | 3.06-4 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only |
| tar | 1.34+dfsg-1.2+deb12u1 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later |
| tzdata | 2026c-0+deb12u1 | deb | public-domain |
| usr-is-merged | 37~deb12u1 | deb | GPL-2.0-only; GPL-2.0-or-later |
| util-linux | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| util-linux-extra | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| zlib1g | 1:1.2.13.dfsg-1 | deb | Zlib |

</details>

### outside-agent

Image ID `b4fb6667c3c541fbe8866a3fed26da064cf666073469accf06131d2c8264845e`; base `Debian GNU/Linux 12 (bookworm)`; **91** packages detected (91 deb).

<details>
<summary>Package list</summary>

| Package | Version | Type | License identifiers found |
| --- | --- | --- | --- |
| adduser | 3.134 | deb | GPL-2.0-only; GPL-2.0-or-later |
| apt | 2.6.1 | deb | BSD-3-Clause; Expat; GPL-2.0-only; GPL-2.0-or-later |
| base-files | 12.4+deb12u15 | deb | NOASSERTION |
| base-passwd | 3.6.1 | deb | GPL-2.0-only; public-domain |
| bash | 5.2.15-2+b13 | deb | BSD-4-Clause-UC; GFDL-1.3-only; GFDL-NIV-1.3; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; Latex2e; MIT-like; permissive |
| bsdutils | 1:2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| ca-certificates | 20250419~deb12u2 | deb | GPL-2.0-only; GPL-2.0-or-later; MPL-2.0 |
| coreutils | 9.1-1 | deb | BSD-4-Clause-UC; FSFULLR; GFDL-1.3-only; GFDL-NIV-1.3; GPL-3.0-only; GPL-3.0-or-later; ISC |
| dash | 0.5.12-2 | deb | BSD-3-Clause; GPL-2.0-only; GPL-2.0-or-later; public-domain |
| debconf | 1.5.82 | deb | BSD-2-Clause |
| debian-archive-keyring | 2023.3+deb12u2 | deb | NOASSERTION |
| debianutils | 5.7-0.5~deb12u1 | deb | GPL-2.0-only; GPL-2.0-or-later; SMAIL-GPL; public-domain |
| diffutils | 1:3.8-4 | deb | FSFAP; FSFULLR; GFDL-1.3-only; GFDL-NIV-1.3; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; X11; public-domain |
| dpkg | 1.21.23 | deb | BSD-2-Clause; GPL-2.0-only; GPL-2.0-or-later; public-domain-s-s-d |
| e2fsprogs | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| findutils | 4.9.0-4 | deb | BSD-3-Clause; FSFAP; FSFULLR; GFDL-1.3-only; GFDL-NIV-1.3+; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; ISC; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; X11; public-domain |
| gcc-12-base | 12.2.0-14+deb12u1 | deb | NOASSERTION |
| gpgv | 2.2.40-1.1+deb12u2 | deb | BSD-3-Clause; CC0-1.0; Expat; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; RFC-Reference; TinySCHEME; permissive |
| grep | 3.8-5 | deb | GPL-3.0-only; GPL-3.0-or-later |
| gzip | 1.12-1 | deb | FSF-manpages; GFDL-1.3+-no-invariant; GFDL-3; GPL-3.0-only; GPL-3.0-or-later |
| hostname | 3.23+nmu1 | deb | GPL-2.0-only |
| init-system-helpers | 1.65.2+deb12u1 | deb | BSD-3-Clause; GPL-2.0-only; GPL-2.0-or-later |
| libacl1 | 2.3.1-3 | deb | GPL-2.0-only; GPL-2.0-or-later; LGPL-2.0-or-later; LGPL-2.1-only |
| libapt-pkg6.0 | 2.6.1 | deb | BSD-3-Clause; Expat; GPL-2.0-only; GPL-2.0-or-later |
| libattr1 | 1:2.5.1-4 | deb | GPL-2.0-only; GPL-2.0-or-later; LGPL-2.0-or-later; LGPL-2.1-only |
| libaudit-common | 1:3.0.9-1 | deb | GPL-1.0-only; GPL-2.0-only; LGPL-2.1-only |
| libaudit1 | 1:3.0.9-1 | deb | GPL-1.0-only; GPL-2.0-only; LGPL-2.1-only |
| libblkid1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libbz2-1.0 | 1.0.8-5+b1 | deb | BSD-variant; GPL-2.0-only |
| libc-bin | 2.36-9+deb12u14 | deb | NOASSERTION |
| libc6 | 2.36-9+deb12u14 | deb | NOASSERTION |
| libcap-ng0 | 0.8.3-1+b3 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; LGPL-2.1-only; LGPL-2.1-or-later |
| libcap2 | 1:2.66-4+deb12u3+b1 | deb | BSD-3-Clause; GPL-2.0-only; GPL-2.0-or-later |
| libcom-err2 | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| libcrypt1 | 1:4.4.33-2 | deb | NOASSERTION |
| libdb5.3 | 5.3.28+dfsg2-1 | deb | Artistic; BSD-3-Clause; BSD-3-clause-fjord; GPL; GPL-3.0-only; MIT-old; MS-PL; Sleepycat; TCL-like; X11; Zlib |
| libdebconfclient0 | 0.270 | deb | BSD-2-Clause; GPL-2.0-only; GPL-2.0-or-later |
| libext2fs2 | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| libffi8 | 3.4.4-1 | deb | Expat; GPL; GPL-2.0-or-later; GPL-3.0-or-later; MPL-1.1; X11; public-domain |
| libgcc-s1 | 12.2.0-14+deb12u1 | deb | NOASSERTION |
| libgcrypt20 | 1.10.1-3+deb12u1 | deb | NOASSERTION |
| libgmp10 | 2:6.2.1+dfsg1-1.1 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later |
| libgnutls30 | 3.7.9-2+deb12u7 | deb | NOASSERTION |
| libgpg-error0 | 1.46-1 | deb | BSD-3-Clause; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; g10-permissive |
| libhogweed6 | 3.8.1-2 | deb | Expat; GAP; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-3.0-or-later; public-domain |
| libidn2-0 | 2.3.3-1+b1 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later; Unicode |
| liblz4-1 | 1.9.4-1 | deb | BSD-2-Clause; GPL-2.0-only; GPL-2.0-or-later |
| liblzma5 | 5.4.1-1+deb12u2 | deb | Autoconf; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; LGPL-2.0-only; LGPL-2.1-only; LGPL-2.1-or-later; PD; PD-debian; config-h; noderivs; permissive-fsf; permissive-nowarranty; probably-PD |
| libmd0 | 1.0.4-2 | deb | BSD-2-Clause; BSD-3-Clause; BSD-3-clause-Aaron-D-Gifford; Beerware; ISC; public-domain-md4; public-domain-md5; public-domain-sha1 |
| libmount1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libnettle8 | 3.8.1-2 | deb | Expat; GAP; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-3.0-or-later; public-domain |
| libp11-kit0 | 0.24.1-2 | deb | Apache-2.0; BSD-3-Clause; ISC; ISC+IBM; LGPL-2.1-only; LGPL-2.1-or-later; permissive-like-automake-output; same-as-rest-of-p11kit |
| libpam-modules | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpam-modules-bin | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpam-runtime | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpam0g | 1.5.2-6+deb12u2 | deb | BSD-3-Clause; BSD-tcp_wrappers; Beerware; GPL; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-2.0-only; LGPL-2.0-or-later; public-domain |
| libpcre2-8-0 | 10.42-1+deb12u2 | deb | BSD-2-Clause; BSD-3-Clause; BSD-3-clause-Cambridge; X11; public-domain |
| libseccomp2 | 2.5.4-1+deb12u1 | deb | LGPL-2.1-only |
| libselinux1 | 3.4-1+b6 | deb | NOASSERTION |
| libsemanage-common | 3.4-1 | deb | NOASSERTION |
| libsemanage2 | 3.4-1+b5 | deb | NOASSERTION |
| libsepol2 | 3.4-2.1 | deb | GPL-2.0-only; GPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; Zlib |
| libsmartcols1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libss2 | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| libssl3 | 3.0.22-1~deb12u1 | deb | Apache-2.0; Artistic; GPL-1.0-only; GPL-1.0-or-later |
| libstdc++6 | 12.2.0-14+deb12u1 | deb | NOASSERTION |
| libsystemd0 | 252.39-1~deb12u2 | deb | CC0-1.0; Expat; GPL-2.0-only; GPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; public-domain |
| libtasn1-6 | 4.19.0-2+deb12u1 | deb | NOASSERTION |
| libtinfo6 | 6.4-4 | deb | BSD-3-Clause; MIT/X11; X11 |
| libudev1 | 252.39-1~deb12u2 | deb | CC0-1.0; Expat; GPL-2.0-only; GPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; public-domain |
| libunistring2 | 1.0-2 | deb | FreeSoftware; GFDL-1.2-only; GFDL-1.2-or-later; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT |
| libuuid1 | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| libxxhash0 | 0.8.1-1 | deb | BSD-2-Clause; GPL-2.0-only |
| libzstd1 | 1.5.4+dfsg2-5 | deb | BSD-3-Clause; Expat; GPL-2.0-only; Zlib |
| login | 1:4.13+dfsg1-1+deb12u2 | deb | BSD-3-Clause; GPL-1.0-only; GPL-2.0-only; GPL-2.0-or-later; public-domain |
| logsave | 1.47.0-2+b2 | deb | Apache-2.0; BSD-3-Clause; GPL; GPL-2.0-only; GPL-2.0-or-later; ISC; Kazlib; LGPL-2.0-only; Latex2e |
| mawk | 1.3.4.20200120-3.1 | deb | CC-BY-3.0; GPL-2.0-only; X11 |
| mount | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| ncurses-base | 6.4-4 | deb | BSD-3-Clause; MIT/X11; X11 |
| ncurses-bin | 6.4-4 | deb | BSD-3-Clause; MIT/X11; X11 |
| openssl | 3.0.22-1~deb12u1 | deb | Apache-2.0; Artistic; GPL-1.0-only; GPL-1.0-or-later |
| passwd | 1:4.13+dfsg1-1+deb12u2 | deb | BSD-3-Clause; GPL-1.0-only; GPL-2.0-only; GPL-2.0-or-later; public-domain |
| perl-base | 5.36.0-7+deb12u4 | deb | Artistic; Artistic-2.0; Artistic-dist; BSD-3-Clause; BSD-3-clause-GENERIC; BSD-3-clause-with-weird-numbering; BSD-4-clause-POWERDOG; BZIP; DONT-CHANGE-THE-GPL; Expat; GPL-1.0-only; GPL-1.0-or-later; GPL-2.0-only; GPL-2.0-or-later; GPL-3+-WITH-BISON-EXCEPTION; HSIEH-BSD; HSIEH-DERIVATIVE; LGPL-2.1-only; REGCOMP; REGCOMP,; RRA-KEEP-THIS-NOTICE; SDBM-PUBLIC-DOMAIN; TEXT-TABS; Unicode; Zlib |
| sed | 4.9-1+deb12u1 | deb | BSD-4-Clause-UC; BSL-1.0; GFDL-1.3-only; GFDL-NIV-1.3+; GPL-3.0-only; GPL-3.0-or-later; ISC; X11; pcre |
| sysvinit-utils | 3.06-4 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only |
| tar | 1.34+dfsg-1.2+deb12u1 | deb | GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL-3.0-only; LGPL-3.0-or-later |
| tzdata | 2026c-0+deb12u1 | deb | public-domain |
| usr-is-merged | 37~deb12u1 | deb | GPL-2.0-only; GPL-2.0-or-later |
| util-linux | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| util-linux-extra | 2.38.1-5+deb12u3 | deb | BSD-3-Clause; BSD-4-Clause; BSLA; GPL-2.0-only; GPL-2.0-or-later; GPL-3.0-only; GPL-3.0-or-later; LGPL; LGPL-2.0-only; LGPL-2.0-or-later; LGPL-2.1-only; LGPL-2.1-or-later; LGPL-3.0-only; LGPL-3.0-or-later; MIT; public-domain |
| zlib1g | 1:1.2.13.dfsg-1 | deb | Zlib |

</details>

### rclone

Image ID `af654060e0e48f59bc80c03dc8468ab16e0f202d245c25da25dbd8350df49b3a`; base `Alpine Linux v3.24`; **266** packages detected (244 go-module, 22 apk).

<details>
<summary>Package list</summary>

| Package | Version | Type | License identifiers found |
| --- | --- | --- | --- |
| alpine-baselayout | 3.7.2-r1 | apk | GPL-2.0-only |
| alpine-baselayout-data | 3.7.2-r1 | apk | GPL-2.0-only |
| alpine-keys | 2.6-r0 | apk | MIT |
| alpine-release | 3.24.2-r0 | apk | MIT |
| apk-tools | 3.0.8-r0 | apk | GPL-2.0-only |
| busybox | 1.37.0-r31 | apk | GPL-2.0-only |
| busybox-binsh | 1.37.0-r31 | apk | GPL-2.0-only |
| ca-certificates | 20260909-r0 | apk | MPL-2.0 AND MIT |
| ca-certificates-bundle | 20260909-r0 | apk | MPL-2.0 AND MIT |
| fuse-common | 3.18.3-r0 | apk | GPL-2.0-only AND LGPL-2.1-only |
| fuse3 | 3.18.3-r0 | apk | GPL-2.0-only AND LGPL-2.1-only |
| fuse3-libs | 3.18.3-r0 | apk | GPL-2.0-only AND LGPL-2.1-only |
| libapk | 3.0.8-r0 | apk | GPL-2.0-only |
| libcrypto3 | 3.5.8-r0 | apk | Apache-2.0 |
| libssl3 | 3.5.8-r0 | apk | Apache-2.0 |
| mailcap | 2.1.54-r0 | apk | Domain; Public |
| musl | 1.2.6-r2 | apk | MIT |
| musl-utils | 1.2.6-r2 | apk | MIT AND BSD-2-Clause AND GPL-2.0-or-later |
| scanelf | 1.3.9-r1 | apk | GPL-2.0-only |
| ssl_client | 1.37.0-r31 | apk | GPL-2.0-only |
| tzdata | 2026e-r0 | apk | Public-Domain |
| zlib | 1.3.2-r0 | apk | Zlib |
| bazil.org/fuse | v0.0.0-20230120002735-62a210ff1fd5 | go-module | NOASSERTION |
| cloud.google.com/go/auth | v0.23.1 | go-module | NOASSERTION |
| cloud.google.com/go/auth/oauth2adapt | v0.2.8 | go-module | NOASSERTION |
| cloud.google.com/go/compute/metadata | v0.9.0 | go-module | NOASSERTION |
| github.com/Azure/azure-sdk-for-go/sdk/azcore | v1.23.1 | go-module | NOASSERTION |
| github.com/Azure/azure-sdk-for-go/sdk/azidentity | v1.14.0 | go-module | NOASSERTION |
| github.com/Azure/azure-sdk-for-go/sdk/internal | v1.12.0 | go-module | NOASSERTION |
| github.com/Azure/azure-sdk-for-go/sdk/storage/azblob | v1.8.1 | go-module | NOASSERTION |
| github.com/Azure/azure-sdk-for-go/sdk/storage/azfile | v1.7.0 | go-module | NOASSERTION |
| github.com/Azure/go-ntlmssp | v0.1.1 | go-module | NOASSERTION |
| github.com/AzureAD/microsoft-authentication-library-for-go | v1.8.0 | go-module | NOASSERTION |
| github.com/FilenCloudDienste/filen-sdk-go | v0.0.39 | go-module | NOASSERTION |
| github.com/Files-com/files-sdk-go/v3 | v3.3.194 | go-module | NOASSERTION |
| github.com/IBM/go-sdk-core/v5 | v5.23.1 | go-module | NOASSERTION |
| github.com/Max-Sum/base32768 | v0.0.0-20230304063302-18e6ce5945fd | go-module | NOASSERTION |
| github.com/ProtonMail/bcrypt | v0.0.0-20211005172633-e235017c1baf | go-module | NOASSERTION |
| github.com/ProtonMail/gluon | v0.17.1-0.20230724134000-308be39be96e | go-module | NOASSERTION |
| github.com/ProtonMail/go-crypto | v1.4.1 | go-module | NOASSERTION |
| github.com/ProtonMail/go-srp | v0.0.7 | go-module | NOASSERTION |
| github.com/ProtonMail/gopenpgp/v3 | v3.4.1 | go-module | NOASSERTION |
| github.com/PuerkitoBio/goquery | v1.12.0 | go-module | NOASSERTION |
| github.com/STARRY-S/zip | v0.2.3 | go-module | NOASSERTION |
| github.com/a1ex3/zstd-seekable-format-go/pkg | v0.10.0 | go-module | NOASSERTION |
| github.com/a8m/tree | v0.0.0-20240104212747-2c8764a5f17e | go-module | NOASSERTION |
| github.com/aalpar/deheap | v1.1.2 | go-module | NOASSERTION |
| github.com/abbot/go-http-auth | v0.4.0 | go-module | NOASSERTION |
| github.com/adrg/xdg | v0.5.3 | go-module | NOASSERTION |
| github.com/anacrolix/dms | v1.7.2 | go-module | NOASSERTION |
| github.com/anacrolix/generics | v0.2.0 | go-module | NOASSERTION |
| github.com/anacrolix/log | v0.17.0 | go-module | NOASSERTION |
| github.com/anchore/go-lzo | v0.1.1 | go-module | NOASSERTION |
| github.com/andybalholm/brotli | v1.2.2 | go-module | NOASSERTION |
| github.com/andybalholm/cascadia | v1.3.4 | go-module | NOASSERTION |
| github.com/apache/arrow-go/v18 | v18.7.0 | go-module | NOASSERTION |
| github.com/appscode/go-querystring | v0.0.0-20170504095604-0126cfb3f1dc | go-module | NOASSERTION |
| github.com/atotto/clipboard | v0.1.4 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2 | v1.43.7 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/aws/protocol/eventstream | v1.7.18 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/config | v1.32.30 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/credentials | v1.19.29 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/feature/ec2/imds | v1.18.30 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/feature/s3/manager | v1.22.34 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/internal/configsources | v1.4.38 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 | v2.7.38 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/internal/v4a | v1.4.39 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding | v1.13.17 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/internal/checksum | v1.9.31 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/internal/presigned-url | v1.13.38 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/internal/s3shared | v1.19.39 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/s3 | v1.107.3 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/signin | v1.4.1 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/sso | v1.32.1 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/ssooidc | v1.37.1 | go-module | NOASSERTION |
| github.com/aws/aws-sdk-go-v2/service/sts | v1.44.1 | go-module | NOASSERTION |
| github.com/aws/smithy-go | v1.27.8 | go-module | NOASSERTION |
| github.com/bahlo/generic-list-go | v0.2.0 | go-module | NOASSERTION |
| github.com/beorn7/perks | v1.0.1 | go-module | NOASSERTION |
| github.com/bodgit/plumbing | v1.3.0 | go-module | NOASSERTION |
| github.com/bodgit/sevenzip | v1.6.5 | go-module | NOASSERTION |
| github.com/bodgit/windows | v1.0.1 | go-module | NOASSERTION |
| github.com/boombuler/barcode | v1.1.0 | go-module | NOASSERTION |
| github.com/bradenaw/juniper | v0.15.3 | go-module | NOASSERTION |
| github.com/buengese/sgzip | v0.1.1 | go-module | NOASSERTION |
| github.com/buger/jsonparser | v1.2.0 | go-module | NOASSERTION |
| github.com/calebcase/tmpfile | v1.0.3 | go-module | NOASSERTION |
| github.com/cespare/xxhash/v2 | v2.3.0 | go-module | NOASSERTION |
| github.com/chilts/sid | v0.0.0-20190607042430-660e94789ec9 | go-module | NOASSERTION |
| github.com/clipperhouse/uax29/v2 | v2.7.0 | go-module | NOASSERTION |
| github.com/cloudflare/circl | v1.6.4 | go-module | NOASSERTION |
| github.com/cloudinary/cloudinary-go/v2 | v2.16.0 | go-module | NOASSERTION |
| github.com/cloudsoda/go-smb2 | v0.0.0-20260701064823-d8c5600d73b8 | go-module | NOASSERTION |
| github.com/cloudsoda/sddl | v0.0.0-20250224235906-926454e91efc | go-module | NOASSERTION |
| github.com/colinmarc/hdfs/v2 | v2.4.0 | go-module | NOASSERTION |
| github.com/coreos/go-semver | v0.3.1 | go-module | NOASSERTION |
| github.com/coreos/go-systemd/v22 | v22.6.0 | go-module | NOASSERTION |
| github.com/cpuguy83/go-md2man/v2 | v2.0.7 | go-module | NOASSERTION |
| github.com/creasty/defaults | v1.8.0 | go-module | NOASSERTION |
| github.com/cronokirby/saferith | v0.33.1-0.20250226174546-1f11f94ce488 | go-module | NOASSERTION |
| github.com/diskfs/go-diskfs | v1.9.4 | go-module | NOASSERTION |
| github.com/dromara/dongle | v1.0.1 | go-module | NOASSERTION |
| github.com/dropbox/dropbox-sdk-go-unofficial/v6 | v6.4.0 | go-module | NOASSERTION |
| github.com/dsnet/compress | v0.0.2-0.20230904184137-39efe44ab707 | go-module | NOASSERTION |
| github.com/emersion/go-message | v0.18.2 | go-module | NOASSERTION |
| github.com/emersion/go-vcard | v0.0.0-20260618161152-d854b7e0e2d3 | go-module | NOASSERTION |
| github.com/felixge/httpsnoop | v1.1.0 | go-module | NOASSERTION |
| github.com/flynn/noise | v1.1.0 | go-module | NOASSERTION |
| github.com/gabriel-vasile/mimetype | v1.4.13 | go-module | NOASSERTION |
| github.com/gdamore/encoding | v1.0.1 | go-module | NOASSERTION |
| github.com/gdamore/tcell/v2 | v2.9.0 | go-module | NOASSERTION |
| github.com/geoffgarside/ber | v1.2.0 | go-module | NOASSERTION |
| github.com/go-chi/chi/v5 | v5.3.1 | go-module | NOASSERTION |
| github.com/go-git/go-billy/v5 | v5.9.0 | go-module | NOASSERTION |
| github.com/go-logr/logr | v1.4.4 | go-module | NOASSERTION |
| github.com/go-logr/stdr | v1.2.2 | go-module | NOASSERTION |
| github.com/go-openapi/errors | v0.22.8 | go-module | NOASSERTION |
| github.com/go-openapi/strfmt | v0.27.0 | go-module | NOASSERTION |
| github.com/go-playground/locales | v0.14.1 | go-module | NOASSERTION |
| github.com/go-playground/universal-translator | v0.18.1 | go-module | NOASSERTION |
| github.com/go-playground/validator/v10 | v10.30.3 | go-module | NOASSERTION |
| github.com/go-resty/resty/v2 | v2.17.2 | go-module | NOASSERTION |
| github.com/go-viper/mapstructure/v2 | v2.5.0 | go-module | NOASSERTION |
| github.com/goccy/go-json | v0.10.6 | go-module | NOASSERTION |
| github.com/gofrs/flock | v0.13.0 | go-module | NOASSERTION |
| github.com/gogo/protobuf | v1.3.2 | go-module | NOASSERTION |
| github.com/golang-jwt/jwt/v4 | v4.5.2 | go-module | NOASSERTION |
| github.com/golang-jwt/jwt/v5 | v5.3.1 | go-module | NOASSERTION |
| github.com/google/btree | v1.1.3 | go-module | NOASSERTION |
| github.com/google/flatbuffers | v25.12.19+incompatible | go-module | NOASSERTION |
| github.com/google/s2a-go | v0.1.9 | go-module | NOASSERTION |
| github.com/google/uuid | v1.6.0 | go-module | NOASSERTION |
| github.com/googleapis/enterprise-certificate-proxy | v0.3.21 | go-module | NOASSERTION |
| github.com/googleapis/gax-go/v2 | v2.23.0 | go-module | NOASSERTION |
| github.com/gorilla/schema | v1.4.1 | go-module | NOASSERTION |
| github.com/hanwen/go-fuse/v2 | v2.10.1 | go-module | NOASSERTION |
| github.com/hashicorp/errwrap | v1.0.0 | go-module | NOASSERTION |
| github.com/hashicorp/go-cleanhttp | v0.5.2 | go-module | NOASSERTION |
| github.com/hashicorp/go-multierror | v1.1.1 | go-module | NOASSERTION |
| github.com/hashicorp/go-retryablehttp | v0.7.8 | go-module | NOASSERTION |
| github.com/hashicorp/go-uuid | v1.0.3 | go-module | NOASSERTION |
| github.com/hashicorp/golang-lru/v2 | v2.0.7 | go-module | NOASSERTION |
| github.com/internxt/rclone-adapter | v0.0.0-20260904121407-404a75f3d031 | go-module | NOASSERTION |
| github.com/jcmturner/aescts/v2 | v2.0.0 | go-module | NOASSERTION |
| github.com/jcmturner/dnsutils/v2 | v2.0.0 | go-module | NOASSERTION |
| github.com/jcmturner/gofork | v1.7.6 | go-module | NOASSERTION |
| github.com/jcmturner/goidentity/v6 | v6.0.1 | go-module | NOASSERTION |
| github.com/jcmturner/gokrb5/v8 | v8.4.4 | go-module | NOASSERTION |
| github.com/jcmturner/rpc/v2 | v2.0.3 | go-module | NOASSERTION |
| github.com/jlaffaye/ftp | v0.2.1-0.20251026020404-6602e981a1bb | go-module | NOASSERTION |
| github.com/jtolio/noiseconn | v0.0.0-20231127013910-f6d9ecbf1de7 | go-module | NOASSERTION |
| github.com/jzelinskie/whirlpool | v0.0.0-20201016144138-0675e54bb004 | go-module | NOASSERTION |
| github.com/klauspost/compress | v1.19.2 | go-module | NOASSERTION |
| github.com/klauspost/cpuid/v2 | v2.4.0 | go-module | NOASSERTION |
| github.com/klauspost/pgzip | v1.2.6 | go-module | NOASSERTION |
| github.com/koofr/go-httpclient | v0.0.0-20240520111329-e20f8f203988 | go-module | NOASSERTION |
| github.com/koofr/go-koofrclient | v0.0.0-20221207135200-cbd7fc9ad6a6 | go-module | NOASSERTION |
| github.com/kr/fs | v0.1.0 | go-module | NOASSERTION |
| github.com/kylelemons/godebug | v1.1.0 | go-module | NOASSERTION |
| github.com/lanrat/extsort | v1.4.2 | go-module | NOASSERTION |
| github.com/leodido/go-urn | v1.4.0 | go-module | NOASSERTION |
| github.com/lpar/calendar | v0.2.0 | go-module | NOASSERTION |
| github.com/lucasb-eyer/go-colorful | v1.4.0 | go-module | NOASSERTION |
| github.com/mailru/easyjson | v0.9.2 | go-module | NOASSERTION |
| github.com/mattn/go-colorable | v0.1.15 | go-module | NOASSERTION |
| github.com/mattn/go-isatty | v0.0.23 | go-module | NOASSERTION |
| github.com/mattn/go-runewidth | v0.0.24 | go-module | NOASSERTION |
| github.com/mholt/archives | v0.1.5 | go-module | NOASSERTION |
| github.com/mikelolasagasti/xz | v1.0.1 | go-module | NOASSERTION |
| github.com/minio/minlz | v1.2.0 | go-module | NOASSERTION |
| github.com/minio/xxml | v0.0.3 | go-module | NOASSERTION |
| github.com/mitchellh/go-homedir | v1.1.0 | go-module | NOASSERTION |
| github.com/moby/sys/mountinfo | v0.7.2 | go-module | NOASSERTION |
| github.com/munnerz/goautoneg | v0.0.0-20191010083416-a7dc8b61c822 | go-module | NOASSERTION |
| github.com/ncw/swift/v2 | v2.0.5 | go-module | NOASSERTION |
| github.com/nwaples/rardecode/v2 | v2.2.5 | go-module | NOASSERTION |
| github.com/oklog/ulid/v2 | v2.1.1 | go-module | NOASSERTION |
| github.com/oracle/oci-go-sdk/v65 | v65.121.0 | go-module | NOASSERTION |
| github.com/panjf2000/ants/v2 | v2.12.1 | go-module | NOASSERTION |
| github.com/patrickmn/go-cache | v2.1.0+incompatible | go-module | NOASSERTION |
| github.com/pengsrc/go-shared | v0.2.1-0.20190131101655-1999055a4a14 | go-module | NOASSERTION |
| github.com/peterh/liner | v1.2.2 | go-module | NOASSERTION |
| github.com/pierrec/lz4/v4 | v4.1.28 | go-module | NOASSERTION |
| github.com/pkg/browser | v0.0.0-20240102092130-5ac0b6a4141c | go-module | NOASSERTION |
| github.com/pkg/errors | v0.9.1 | go-module | NOASSERTION |
| github.com/pkg/sftp | v1.13.11 | go-module | NOASSERTION |
| github.com/pkg/xattr | v0.4.12 | go-module | NOASSERTION |
| github.com/pquerna/otp | v1.5.0 | go-module | NOASSERTION |
| github.com/prometheus/client_golang | v1.23.2 | go-module | NOASSERTION |
| github.com/prometheus/client_model | v0.6.2 | go-module | NOASSERTION |
| github.com/prometheus/common | v0.70.0 | go-module | NOASSERTION |
| github.com/prometheus/procfs | v0.21.1 | go-module | NOASSERTION |
| github.com/putdotio/go-putio/putio | v0.0.0-20200123120452-16d982cac2b8 | go-module | NOASSERTION |
| github.com/rasky/go-xdr | v0.0.0-20170124162913-1a41d1a06c93 | go-module | NOASSERTION |
| github.com/rclone/Proton-API-Bridge | v1.0.5 | go-module | NOASSERTION |
| github.com/rclone/go-proton-api | v1.0.4 | go-module | NOASSERTION |
| github.com/rclone/gofakes3 | v0.0.10 | go-module | NOASSERTION |
| github.com/rclone/rclone | v1.75.2+dirty | go-module | NOASSERTION |
| github.com/relvacode/iso8601 | v1.7.0 | go-module | NOASSERTION |
| github.com/rfjakob/eme | v1.2.0 | go-module | NOASSERTION |
| github.com/rivo/uniseg | v0.4.7 | go-module | NOASSERTION |
| github.com/russross/blackfriday/v2 | v2.1.0 | go-module | NOASSERTION |
| github.com/ryszard/goskiplist | v0.0.0-20150312221310-2dfbae5fcf46 | go-module | NOASSERTION |
| github.com/sabhiram/go-gitignore | v0.0.0-20210923224102-525f6e181f06 | go-module | NOASSERTION |
| github.com/samber/lo | v1.53.0 | go-module | NOASSERTION |
| github.com/shirou/gopsutil/v4 | v4.26.6 | go-module | NOASSERTION |
| github.com/sirupsen/logrus | v1.9.4 | go-module | NOASSERTION |
| github.com/skratchdot/open-golang | v0.0.0-20200116055534-eef842397966 | go-module | NOASSERTION |
| github.com/sony/gobreaker/v2 | v2.4.0 | go-module | NOASSERTION |
| github.com/sorairolake/lzip-go | v0.3.8 | go-module | NOASSERTION |
| github.com/spacemonkeygo/monkit/v3 | v3.0.25-0.20251022131615-eb24eb109368 | go-module | NOASSERTION |
| github.com/spf13/afero | v1.15.0 | go-module | NOASSERTION |
| github.com/spf13/cobra | v1.10.2 | go-module | NOASSERTION |
| github.com/spf13/pflag | v1.0.10 | go-module | NOASSERTION |
| github.com/stangelandcl/ppmd | v0.1.1 | go-module | NOASSERTION |
| github.com/stretchr/testify | v1.12.1 | go-module | NOASSERTION |
| github.com/t3rm1n4l/go-mega | v0.0.0-20260717075258-c6acd6a5bd04 | go-module | NOASSERTION |
| github.com/tyler-smith/go-bip39 | v1.1.0 | go-module | NOASSERTION |
| github.com/ulikunitz/xz | v0.5.15 | go-module | NOASSERTION |
| github.com/unknwon/goconfig | v1.0.0 | go-module | NOASSERTION |
| github.com/willscott/go-nfs | v0.0.4 | go-module | NOASSERTION |
| github.com/willscott/go-nfs-client | v0.0.0-20251022144359-801f10d98886 | go-module | NOASSERTION |
| github.com/wk8/go-ordered-map/v2 | v2.1.8 | go-module | NOASSERTION |
| github.com/xanzy/ssh-agent | v0.3.3 | go-module | NOASSERTION |
| github.com/youmark/pkcs8 | v0.0.0-20240726163527-a2c0da244d78 | go-module | NOASSERTION |
| github.com/yunify/qingstor-sdk-go/v3 | v3.2.0 | go-module | NOASSERTION |
| github.com/zeebo/blake3 | v0.2.4 | go-module | NOASSERTION |
| github.com/zeebo/errs | v1.4.0 | go-module | NOASSERTION |
| github.com/zeebo/xxh3 | v1.1.0 | go-module | NOASSERTION |
| go.etcd.io/bbolt | v1.5.0 | go-module | NOASSERTION |
| go.opentelemetry.io/auto/sdk | v1.2.1 | go-module | NOASSERTION |
| go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp | v0.70.0 | go-module | NOASSERTION |
| go.opentelemetry.io/otel | v1.45.0 | go-module | NOASSERTION |
| go.opentelemetry.io/otel/metric | v1.45.0 | go-module | NOASSERTION |
| go.opentelemetry.io/otel/trace | v1.45.0 | go-module | NOASSERTION |
| go.yaml.in/yaml/v2 | v2.4.4 | go-module | NOASSERTION |
| go.yaml.in/yaml/v3 | v3.0.5 | go-module | NOASSERTION |
| go4.org | v0.0.0-20260112195520-a5071408f32f | go-module | NOASSERTION |
| goftp.io/server/v2 | v2.0.3 | go-module | NOASSERTION |
| golang.org/x/crypto | v0.57.0 | go-module | NOASSERTION |
| golang.org/x/exp | v0.0.0-20260813180055-c1d0aacb2297 | go-module | NOASSERTION |
| golang.org/x/image | v0.45.0 | go-module | NOASSERTION |
| golang.org/x/net | v0.60.0 | go-module | NOASSERTION |
| golang.org/x/oauth2 | v0.36.0 | go-module | NOASSERTION |
| golang.org/x/sync | v0.23.0 | go-module | NOASSERTION |
| golang.org/x/sys | v0.48.0 | go-module | NOASSERTION |
| golang.org/x/term | v0.46.0 | go-module | NOASSERTION |
| golang.org/x/text | v0.42.0 | go-module | NOASSERTION |
| golang.org/x/time | v0.15.0 | go-module | NOASSERTION |
| google.golang.org/api | v0.293.0 | go-module | NOASSERTION |
| google.golang.org/genproto/googleapis/rpc | v0.0.0-20260817212433-ac3dfec99bb1 | go-module | NOASSERTION |
| google.golang.org/grpc | v1.85.0-dev.0.20260825072537-93e31b48545e | go-module | NOASSERTION |
| google.golang.org/protobuf | v1.36.12 | go-module | NOASSERTION |
| gopkg.in/natefinch/lumberjack.v2 | v2.2.1 | go-module | NOASSERTION |
| gopkg.in/validator.v2 | v2.0.1 | go-module | NOASSERTION |
| gopkg.in/yaml.v2 | v2.4.0 | go-module | NOASSERTION |
| gopkg.in/yaml.v3 | v3.0.1 | go-module | NOASSERTION |
| moul.io/http2curl/v2 | v2.3.0 | go-module | NOASSERTION |
| sigs.k8s.io/yaml | v1.6.0 | go-module | NOASSERTION |
| stdlib | go1.27.2 | go-module | BSD-3-Clause |
| storj.io/common | v0.0.0-20260629224719-ba1bff0a7846 | go-module | NOASSERTION |
| storj.io/drpc | v1.0.0 | go-module | NOASSERTION |
| storj.io/eventkit | v0.0.0-20260716074419-6861a92e2aa5 | go-module | NOASSERTION |
| storj.io/infectious | v0.0.2 | go-module | NOASSERTION |
| storj.io/picobuf | v0.0.4 | go-module | NOASSERTION |
| storj.io/uplink | v1.14.3 | go-module | NOASSERTION |

</details>

## Refresh

Needs rootless Podman, network access, and a prior `scripts/build.sh` (it leaves the binaries in the `a2a-target` volume). Everything else runs in disposable containers:

```sh
scripts/build.sh
scripts/generate-sbom.py
```

Review the diff of `sbom.md` before committing; changed licenses deserve a look.
