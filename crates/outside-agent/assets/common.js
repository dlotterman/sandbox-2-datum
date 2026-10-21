// Helpers shared by the dashboard and the about page.
//
// Plain browser JavaScript: no build step, no libraries. Everything that puts text on the page goes through
// `textContent` or text nodes, never `innerHTML`, so nothing that arrives from the API can inject markup.

/** Find an element by its id. */
const byId = (id) => document.getElementById(id);

/**
 * Create an element. `properties` are assigned onto it (className, textContent, ...), and `children` may be
 * nodes, strings, or arrays of those:  element("div", {className: "card"}, titleNode, [rowA, rowB])
 */
function element(tag, properties = {}, ...children) {
  const created = Object.assign(document.createElement(tag), properties);
  children.flat().forEach((child) => created.append(child));
  return created;
}

/** An element that holds only text, with an optional CSS class. */
function textElement(tag, className, text) {
  return element(tag, { className, textContent: text });
}

/**
 * The non-empty values joined with a separator:
 * joinNonEmpty(["Los Angeles", "", "CA"], ", ") gives "Los Angeles, CA".
 */
function joinNonEmpty(values, separator) {
  return values.filter(Boolean).join(separator);
}

/** A definition list (`<dl>`) from an object: keys become terms, values become descriptions ("—" if missing). */
function definitionList(entries) {
  const list = element("dl");
  for (const [term, description] of Object.entries(entries)) {
    list.append(textElement("dt", "", term), textElement("dd", "", description ?? "—"));
  }
  return list;
}

// ---- times. The API sends Unix seconds.

/** A date and time for people, in the viewer's locale and timezone. */
function formatTime(unixSeconds) {
  return unixSeconds ? new Date(unixSeconds * 1000).toLocaleString() : "never";
}

// "12s ago" up to 90 seconds, then "5m ago" up to 90 minutes, then "2h ago".
const SHOW_SECONDS_BELOW = 90;
const SHOW_MINUTES_BELOW = 90 * 60;

/** How long ago `unixSeconds` was, in the coarsest unit that still reads naturally. */
function timeAgo(unixSeconds) {
  if (!unixSeconds) return "never";
  const secondsAgo = Math.max(0, Math.floor(Date.now() / 1000 - unixSeconds));
  if (secondsAgo < SHOW_SECONDS_BELOW) return `${secondsAgo}s ago`;
  if (secondsAgo < SHOW_MINUTES_BELOW) return `${Math.round(secondsAgo / 60)}m ago`;
  return `${Math.round(secondsAgo / 3600)}h ago`;
}

// ---- text from the language model

/**
 * Turn the small amount of markdown the model uses into safe elements: **bold** and `code` become
 * <strong> and <code>; everything else stays plain text. Returns an array of nodes.
 */
function richText(text) {
  const pieces = text.split(/(\*\*[^*]+\*\*|`[^`]+`)/).filter(Boolean);
  return pieces.map((piece) => {
    if (piece.startsWith("**")) return element("strong", { textContent: piece.slice(2, -2) });
    if (piece.startsWith("`")) return element("code", { textContent: piece.slice(1, -1) });
    return document.createTextNode(piece);
  });
}
