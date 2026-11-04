import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Script } from "node:vm";

const html = await readFile(new URL("../design/squelch-ui-prototype.html", import.meta.url), "utf8");

test("concept is self-contained HTML with syntactically valid script", () => {
  assert.match(html, /^<!doctype html>/i);
  assert.match(html, /<html lang="en">/);
  assert.match(html, /<meta name="viewport"/);
  assert.match(html, /<title>Squelch/);
  assert.doesNotMatch(html, /<script[^>]+src=|<link[^>]+href=/i);
  const scripts = [...html.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/gi)];
  assert.equal(scripts.length, 1);
  assert.doesNotThrow(() => new Script(scripts[0][1], { filename: "squelch-ui-prototype.html" }));
});

test("five modes and four decks have explicit identities", () => {
  for (const mode of ["perform", "produce", "shape", "mix", "master"]) {
    assert.ok(html.includes('data-mode="' + mode + '"'));
    assert.ok(html.includes('data-view="' + mode + '"'));
  }
  assert.ok(html.includes("['A','B','C','D']"));
  for (const deck of ["A","B","C","D"]) assert.ok(html.includes(deck + ":"), deck);
  assert.match(html, /stageBtn/);
  assert.match(html, /id="muteAll"/);
});

test("concept clearly discloses missing audio and supports accessibility", () => {
  for (const phrase of ["INTERACTIVE DESIGN CONCEPT · NO AUDIO","AUDIO DEVICE: NOT CONNECTED","CUE: UNAVAILABLE","NO AUDIO OUTPUT","Simulation only · No sound is generated.","LUFS --","not measured","No files are loaded or decoded"]) assert.ok(html.includes(phrase),phrase);
  assert.doesNotMatch(html, /\b(?:AudioContext|webkitAudioContext|mediaDevices|getUserMedia|fetch\(|WebSocket\(|localStorage\.)/);
  assert.match(html,/aria-live="polite"/);
  assert.match(html,/:focus-visible/);
  assert.match(html,/prefers-reduced-motion/);
  assert.match(html,/@media\(max-width:850px\)/);
  assert.match(html,/@media\(max-width:420px\)/);
  assert.match(html,/e\.altKey/);
  assert.match(html,/e\.key==='Escape'/);
  assert.match(html,/aria-pressed/);
});
