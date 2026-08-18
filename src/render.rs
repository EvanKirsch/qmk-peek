pub struct ThumbCluster {
    pub top: (String, String),
    pub bottom: (Vec<String>, Vec<String>),
}

pub struct KeyGrid {
    pub label: String,
    pub rows: Vec<(Vec<String>, Vec<String>)>,
    pub thumb: Option<ThumbCluster>,
}

pub struct RenderStyle {
    pub cell_w: usize,
    pub thumb_w: usize,
    pub thumb_indent: usize,
    pub thumb_gap: usize,
}

/// Marks a grid position that has no physical key (e.g. a staggered column
/// that only exists in some rows), so it's rendered as blank space instead
/// of an empty boxed cell.
pub const NO_KEY: &str = "\0";

fn exists(label: &str) -> bool {
    label != NO_KEY
}

fn center(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        return s.to_string();
    }
    let total_pad = width - len;
    let left = total_pad / 2;
    let right = total_pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

fn cell(label: &str, width: usize) -> String {
    let truncated: String = label.chars().take(width).collect();
    center(&truncated, width)
}

fn hline(cells: &[String], left: char, mid: char, right: char, w: usize) -> String {
    let n = cells.len();
    let mut out = String::new();
    for i in 0..n {
        let here = exists(&cells[i]);
        let cap = if i == 0 {
            if here {
                left
            } else {
                ' '
            }
        } else if here || exists(&cells[i - 1]) {
            mid
        } else {
            ' '
        };
        out.push(cap);
        out.push_str(&if here { "-".repeat(w) } else { " ".repeat(w) });
    }
    out.push(if exists(&cells[n - 1]) { right } else { ' ' });
    out
}

fn content(cells: &[String], w: usize) -> String {
    let n = cells.len();
    let mut out = String::new();
    for i in 0..n {
        let here = exists(&cells[i]);
        let bar = if i == 0 {
            if here {
                '|'
            } else {
                ' '
            }
        } else if here || exists(&cells[i - 1]) {
            '|'
        } else {
            ' '
        };
        out.push(bar);
        out.push_str(&if here {
            cell(&cells[i], w)
        } else {
            " ".repeat(w)
        });
    }
    out.push(if exists(&cells[n - 1]) { '|' } else { ' ' });
    out
}

fn row(l: &[String], r: &[String], w: usize) -> String {
    format!("   {}   {}", content(l, w), content(r, w))
}

pub fn render(grid: &KeyGrid, style: &RenderStyle) -> String {
    let w = style.cell_w;
    let mut out: Vec<String> = vec![grid.label.clone()];

    let n_rows = grid.rows.len();
    for (i, (l, r)) in grid.rows.iter().enumerate() {
        if i == 0 {
            out.push(format!(
                "   {}   {}",
                hline(l, ',', '+', ',', w),
                hline(r, ',', '+', ',', w)
            ));
        }
        out.push(row(l, r, w));
        if i == n_rows - 1 {
            out.push(format!(
                "   {}   {}",
                hline(l, '`', '+', '`', w),
                hline(r, '`', '+', '`', w)
            ));
        } else {
            out.push(format!(
                "   {}   {}",
                hline(l, '+', '+', '+', w),
                hline(r, '+', '+', '+', w)
            ));
        }
    }

    if let Some(thumb) = &grid.thumb {
        out.push(String::new());

        let tw = style.thumb_w;
        let span = 3 * (tw + 1) - 1;
        let indent = " ".repeat(style.thumb_indent);
        let gap = " ".repeat(style.thumb_gap);

        let thumb_hline = |left: &str, mid: &str, right: &str| -> String {
            let seg = "-".repeat(tw);
            let segs = vec![seg; 3];
            format!("{}{}{}", left, segs.join(mid), right)
        };

        out.push(
            format!(
                "{}{}{}{}{}{}{}",
                indent,
                ",",
                "-".repeat(span),
                ",",
                gap,
                ",",
                "-".repeat(span)
            ) + ",",
        );
        out.push(format!(
            "{}|{}|{}|{}|",
            indent,
            center(&thumb.top.0, span),
            gap,
            center(&thumb.top.1, span)
        ));
        out.push(format!(
            "{}{}{}{}",
            indent,
            thumb_hline("+", "-", "+"),
            gap,
            thumb_hline("+", "-", "+")
        ));
        let bl = format!(
            "|{}|",
            thumb
                .bottom
                .0
                .iter()
                .map(|x| cell(x, tw))
                .collect::<Vec<_>>()
                .join("|")
        );
        let br = format!(
            "|{}|",
            thumb
                .bottom
                .1
                .iter()
                .map(|x| cell(x, tw))
                .collect::<Vec<_>>()
                .join("|")
        );
        out.push(format!("{}{}{}{}", indent, bl, gap, br));
        out.push(format!(
            "{}{}{}{}",
            indent,
            thumb_hline("`", "-", "`"),
            gap,
            thumb_hline("`", "-", "`")
        ));
    }

    out.join("\n")
}
