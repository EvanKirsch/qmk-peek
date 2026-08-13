use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static MOD_ABBR: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("MOD_LCTL", "CTL"),
        ("MOD_RCTL", "CTL"),
        ("MOD_LSFT", "SHIFT"),
        ("MOD_RSFT", "SHIFT"),
        ("MOD_LALT", "ALT"),
        ("MOD_RALT", "ALT"),
        ("MOD_LGUI", "MOD"),
        ("MOD_RGUI", "MOD"),
    ])
});

static KC_ABBR: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("EQUAL", "="),
        ("MINUS", "-"),
        ("SLASH", "/"),
        ("BSLS", "\\"),
        ("COMMA", ","),
        ("DOT", "."),
        ("SCLN", ";"),
        ("QUOTE", "'"),
        ("GRAVE", "`"),
        ("LBRC", "["),
        ("RBRC", "]"),
        ("LCBR", "{"),
        ("RCBR", "}"),
        ("LPRN", "("),
        ("RPRN", ")"),
        ("PIPE", "|"),
        ("EXLM", "!"),
        ("AT", "@"),
        ("HASH", "#"),
        ("DLR", "$"),
        ("PERC", "%"),
        ("CIRC", "^"),
        ("AMPR", "&"),
        ("ASTR", "*"),
        ("TILD", "~"),
        ("KP_PLUS", "+"),
        ("SPACE", "SPACE"),
        ("BSPC", "BSPACE"),
        ("ENTER", "ENTER"),
        ("TAB", "TAB"),
        ("ESCAPE", "ESC"),
        ("DELETE", "DEL"),
        ("INSERT", "INS"),
        ("CAPS", "CAPS"),
        ("LEFT_SHIFT", "SHIFT"),
        ("RIGHT_SHIFT", "SHIFT"),
        ("LEFT_GUI", "MOD"),
        ("RIGHT_GUI", "MOD"),
        ("LEFT_ALT", "ALT"),
        ("RIGHT_ALT", "ALT"),
        ("LEFT_CTRL", "CTL"),
        ("RIGHT_CTRL", "CTL"),
        ("LEFT_CTL", "CTL"),
        ("RIGHT_CTL", "CTL"),
        ("LEFT", "LEFT"),
        ("RIGHT", "RIGHT"),
        ("UP", "UP"),
        ("DOWN", "DOWN"),
        ("AUDIO_VOL_UP", "VOL+"),
        ("AUDIO_VOL_DOWN", "VOL-"),
        ("AUDIO_MUTE", "MUTE"),
        ("MEDIA_PLAY_PAUSE", "PLAY"),
        ("MEDIA_PREV_TRACK", "PREV"),
        ("MEDIA_NEXT_TRACK", "NEXT"),
        ("WWW_BACK", "BACK"),
        ("MS_BTN1", "MS1"),
        ("MS_BTN2", "MS2"),
    ])
});

static CUSTOM_ABBR: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("KC_TRANSPARENT", ""),
        ("KC_NO", ""),
        ("XXXXXXX", ""),
        ("QK_BOOT", "BOOT"),
        ("RGB_TOG", "RGBTOG"),
        ("RGB_MODE_FORWARD", "RGB"),
        ("RGB_VAD", "RGB_-"),
        ("RGB_VAI", "RGB_+"),
        ("RGB_HUD", "HUE-"),
        ("RGB_HUI", "HUE+"),
        ("RGB_SLD", "RGBSLD"),
        ("TOGGLE_LAYER_COLOR", "LYRCLR"),
        ("AU_TOGG", "AUTOG"),
        ("MU_TOGG", "MUTOG"),
        ("MU_NEXT", "MUNXT"),
    ])
});

fn format_base(tok: &str) -> String {
    let tok = tok.trim();
    if let Some(v) = CUSTOM_ABBR.get(tok) {
        return v.to_string();
    }
    if let Some(suffix) = tok.strip_prefix("KC_") {
        if let Some(v) = KC_ABBR.get(suffix) {
            return v.to_string();
        }
        if suffix.chars().count() == 1 {
            return suffix.to_string();
        }
        return suffix.chars().take(7).collect();
    }
    // HSV_x_y_z or other custom user keycodes - just shorten
    tok.chars().take(7).collect()
}

static MT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^MT\((\w+),\s*(.+)\)$").unwrap());
static LT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^LT\((\d+),\s*(.+)\)$").unwrap());
static LAYER_FN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(TG|MO|TO|DF|OSL)\((\d+)\)$").unwrap());

pub fn format_keycode(tok: &str) -> String {
    let tok = tok.trim();
    if let Some(caps) = MT_RE.captures(tok) {
        let modifier = &caps[1];
        let kc = &caps[2];
        let m = MOD_ABBR.get(modifier).copied().unwrap_or(modifier);
        return format!("{}({})", format_base(kc), m);
    }
    if let Some(caps) = LT_RE.captures(tok) {
        let layer = &caps[1];
        let kc = &caps[2];
        return format!("{}(L{})", format_base(kc), layer);
    }
    if let Some(caps) = LAYER_FN_RE.captures(tok) {
        return format!("LYR-{}", &caps[2]);
    }
    format_base(tok)
}
