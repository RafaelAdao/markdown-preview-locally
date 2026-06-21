// Tests for the client-side relative-link behaviour defined in assets/app.js.
// Run via: node tests/links.js
// Integrated into `cargo test` through tests/js_links.rs.

'use strict';

var failed = 0;

function assert(condition, message) {
  if (!condition) {
    console.error('FAIL:', message);
    failed++;
  } else {
    console.log('ok  :', message);
  }
}

function assertEqual(actual, expected, message) {
  if (actual !== expected) {
    console.error('FAIL:', message);
    console.error('     expected:', JSON.stringify(expected));
    console.error('     actual  :', JSON.stringify(actual));
    failed++;
  } else {
    console.log('ok  :', message);
  }
}

// ── Extracted from assets/app.js (must stay in sync) ─────────────────────────

function resolveRelativePath(base, rel) {
  var dir = base.replace(/[^/]*$/, '');
  var parts = (dir + rel).split('/');
  var out = [];
  for (var i = 0; i < parts.length; i++) {
    var p = parts[i];
    if (p === '..') out.pop();
    else if (p !== '' && p !== '.') out.push(p);
  }
  return out.join('/');
}

function shouldIntercept(href) {
  if (!href) return false;
  if (href.startsWith('#')) return false;
  if (/^(https?:\/\/|mailto:|\/\/)/.test(href)) return false;
  return true;
}

function extractFilePart(href) {
  var hashIdx = href.indexOf('#');
  return hashIdx !== -1 ? href.slice(0, hashIdx) : href;
}

// ── resolveRelativePath ───────────────────────────────────────────────────────

assertEqual(
  resolveRelativePath('README.md', 'docs/file.md'),
  'docs/file.md',
  'link from root file into subdirectory'
);

assertEqual(
  resolveRelativePath('docs/README.md', 'file.md'),
  'docs/file.md',
  'sibling file in the same directory'
);

assertEqual(
  resolveRelativePath('docs/README.md', '../other.md'),
  'other.md',
  'one level up from a subdirectory'
);

assertEqual(
  resolveRelativePath('a/b/c.md', '../../d.md'),
  'd.md',
  'two levels up'
);

assertEqual(
  resolveRelativePath('docs/sub/file.md', '../sibling.md'),
  'docs/sibling.md',
  'one level up staying inside parent directory'
);

assertEqual(
  resolveRelativePath('docs/sub/file.md', './local.md'),
  'docs/sub/local.md',
  'explicit current-directory prefix'
);

// ── shouldIntercept ───────────────────────────────────────────────────────────

assert(shouldIntercept('docs/file.md'),        'relative path is intercepted');
assert(shouldIntercept('../other.md'),          'parent-relative path is intercepted');
assert(shouldIntercept('/abs/path.md'),         'absolute path (no protocol) is intercepted');
assert(!shouldIntercept('https://example.com'), 'https link is NOT intercepted');
assert(!shouldIntercept('http://localhost'),    'http link is NOT intercepted');
assert(!shouldIntercept('mailto:a@b.com'),      'mailto link is NOT intercepted');
assert(!shouldIntercept('//cdn.example.com'),   'protocol-relative URL is NOT intercepted');
assert(!shouldIntercept('#section'),            'fragment-only link is NOT intercepted');
assert(!shouldIntercept(''),                    'empty href is NOT intercepted');
assert(!shouldIntercept(null),                  'null href is NOT intercepted');

// ── extractFilePart ───────────────────────────────────────────────────────────

assertEqual(extractFilePart('docs/file.md'),           'docs/file.md', 'no fragment');
assertEqual(extractFilePart('docs/file.md#section'),   'docs/file.md', 'fragment stripped');
assertEqual(extractFilePart('docs/file.md#a#b'),       'docs/file.md', 'only first # stripped');

// ── Result ────────────────────────────────────────────────────────────────────

if (failed > 0) {
  console.error('\n' + failed + ' test(s) failed.');
  process.exit(1);
} else {
  console.log('\nAll tests passed.');
}
