use crate::keycodes::format_keycode;
use crate::render::{self, KeyGrid, RenderStyle, ThumbCluster};

pub const MACRO: &str = "LAYOUT_moonlander";
pub const ARG_COUNT: usize = 72;

const STYLE: RenderStyle = RenderStyle {
    cell_w: 6,
    thumb_w: 6,
    thumb_indent: 30,
    thumb_gap: 18,
};

fn layout(idx: usize, tokens: &[String]) -> Result<KeyGrid, String> {
    if tokens.len() != ARG_COUNT {
        return Err(format!(
            "_{}\n  (skipped: expected {} keys, found {} - not a standard LAYOUT_moonlander layer)\n",
            idx, ARG_COUNT, tokens.len()
        ));
    }

    let labels: Vec<String> = tokens.iter().map(|t| format_keycode(t)).collect();

    let r1l = labels[0..7].to_vec();
    let r1r = labels[7..14].to_vec();

    let r2l = labels[14..21].to_vec();
    let r2r = labels[21..28].to_vec();
    let r3l = labels[28..35].to_vec();
    let r3r = labels[35..42].to_vec();

    let mut r4l: Vec<String> = labels[42..48].to_vec();
    r4l.push(String::new());
    let mut r4r: Vec<String> = vec![String::new()];
    r4r.extend_from_slice(&labels[48..54]);

    let mut r5l: Vec<String> = labels[54..59].to_vec();
    r5l.push(String::new());
    r5l.push(String::new());
    let mut r5r: Vec<String> = vec![String::new(), String::new()];
    r5r.extend_from_slice(&labels[61..66]);

    let thumb_tl = labels[59].clone();
    let thumb_tr = labels[60].clone();
    let thumb_bl = labels[66..69].to_vec();
    let thumb_br: Vec<String> = labels[69..72].iter().rev().cloned().collect();

    Ok(KeyGrid {
        label: format!("_{}", idx),
        rows: vec![(r1l, r1r), (r2l, r2r), (r3l, r3r), (r4l, r4r), (r5l, r5r)],
        thumb: Some(ThumbCluster {
            top: (thumb_tl, thumb_tr),
            bottom: (thumb_bl, thumb_br),
        }),
    })
}

pub fn render(idx: usize, tokens: &[String]) -> String {
    match layout(idx, tokens) {
        Ok(grid) => render::render(&grid, &STYLE),
        Err(msg) => msg,
    }
}
