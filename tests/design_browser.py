#!/usr/bin/env python3
"""Standalone visual browser acceptance for Squelch's silent UI concept.

This checks the *design prototype*, not audio playback or a product release.
"""
from pathlib import Path
from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parent.parent
ORIGIN = (ROOT / "design" / "squelch-ui-prototype.html").as_uri()
OUTPUT = ROOT / "browser-design-qa"
OUTPUT.mkdir(parents=True, exist_ok=True)
VIEWPORTS = [(320, 640), (390, 844), (768, 900), (1440, 960)]
MODES = ("perform", "produce", "shape", "mix", "master")


def verify(browser_type, browser_name, width, height):
    browser = browser_type.launch(headless=True)
    page = browser.new_page(viewport={"width": width, "height": height}, device_scale_factor=1)
    errors = []
    page.on("pageerror", lambda err: errors.append(str(err)))
    try:
        response = page.goto(ORIGIN, wait_until="load")
        assert response is not None and response.ok, (browser_name, width, "file load")
        assert page.title() == "Squelch — Unified Studio UI Concept"
        assert page.get_by_text("INTERACTIVE DESIGN CONCEPT · NO AUDIO").is_visible()
        assert page.locator(".deck").count() == 4, (browser_name, width, "four decks")
        assert page.locator('[data-view="perform"]').is_visible()
        assert page.locator("#notice").is_visible()

        def verify_overflow(stage):
            d = page.evaluate("""() => ({
              root: document.documentElement.scrollWidth,
              inner: window.innerWidth,
              view: document.querySelector("#surface").scrollWidth,
              client: document.querySelector("#surface").clientWidth
            })""")
            assert d["root"] <= d["inner"] + 2, (browser_name, width, stage, "horizontal page overflow", d)
            assert d["view"] <= d["client"] + 2, (browser_name, width, stage, "surface overflow", d)

        verify_overflow("initial")
        for mode in MODES:
            button = page.locator('[data-mode="' + mode + '"]')
            button.click()
            assert button.get_attribute("aria-pressed") == "true", (browser_name, width, mode)
            assert page.locator('[data-view="' + mode + '"]').is_visible()
            assert page.locator('[data-view]').filter(visible=True).count() == 1
            verify_overflow(mode)
            if mode in ("perform", "produce", "mix"):
                page.screenshot(path=str(OUTPUT / (browser_name + "-" + str(width) + "-" + mode + ".png")),
                                animations="disabled", full_page=True)

        page.locator('[data-mode="perform"]').click()
        select_b = page.locator('button[data-act="select"][data-deck="B"]')
        select_b.focus()
        select_b.press("Enter")
        assert page.locator("#inspectorSelection").text_content() == "Deck B"
        assert page.evaluate("document.activeElement?.dataset?.deck") == "B", (browser_name, width, "deck focus restoration")

        play_b = page.locator('button[data-act="play"][data-deck="B"]')
        play_b.focus()
        play_b.press("Enter")
        assert page.locator('button[data-act="play"][data-deck="B"]').get_attribute("aria-pressed") == "true"
        assert page.evaluate("document.activeElement?.dataset?.act") == "play", (browser_name, width, "transport focus restoration")
        assert page.locator("#globalTransport").text_content() == "PAUSE DEMO"
        assert "no sound is generated" in page.locator("#notice").text_content().lower()

        stage_button = page.locator("#stageBtn")
        stage_button.click()
        assert stage_button.get_attribute("aria-pressed") == "true"
        assert "stage" in page.locator("#app").get_attribute("class")
        verify_overflow("stage")
        page.locator('[data-mode="produce"]').click()
        assert "stage" not in page.locator("#app").get_attribute("class"), (browser_name, width, "stage lock exit")

        page.emulate_media(reduced_motion="reduce")
        page.locator('[data-mode="perform"]').click()
        duration = page.locator(".deck-meters i").first.evaluate("el => getComputedStyle(el).transitionDuration")
        assert duration in ("0s", "0.0s"), (browser_name, width, "reduced motion", duration)
        assert not errors, (browser_name, width, "console errors", errors)
        print("PASS", browser_name, width, height,
              "5 modes, 4 decks, navigation, keyboard, focus, stage lock, overflow, reduced motion")
    finally:
        page.close()
        browser.close()


def main():
    with sync_playwright() as p:
        for name in ("chromium", "webkit"):
            browser_type = getattr(p, name)
            for width, height in VIEWPORTS:
                verify(browser_type, name, width, height)
    print("PASS: silent UI concept cross-browser acceptance complete")


if __name__ == "__main__":
    main()
