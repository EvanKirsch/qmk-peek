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

fn hline(n: usize, left: &str, mid: &str, right: &str, w: usize) -> String {
    let seg = "-".repeat(w);
    let segs = vec![seg; n];
    format!("{}{}{}", left, segs.join(mid), right)
}

fn row(l: &[String], r: &[String], w: usize) -> String {
    let left = format!(
        "|{}|",
        l.iter().map(|x| cell(x, w)).collect::<Vec<_>>().join("|")
    );
    let right = format!(
        "|{}|",
        r.iter().map(|x| cell(x, w)).collect::<Vec<_>>().join("|")
    );
    format!("   {}   {}", left, right)
}

pub fn render(grid: &KeyGrid, style: &RenderStyle) -> String {
    let w = style.cell_w;
    let mut out: Vec<String> = vec![grid.label.clone()];

    let n_rows = grid.rows.len();
    for (i, (l, r)) in grid.rows.iter().enumerate() {
        if i == 0 {
            out.push(format!(
                "   {}   {}",
                hline(l.len(), ",", "+", ",", w),
                hline(r.len(), ",", "+", ",", w)
            ));
        }
        out.push(row(l, r, w));
        if i == n_rows - 1 {
            out.push(format!(
                "   {}   {}",
                hline(l.len(), "`", "+", "`", w),
                hline(r.len(), "`", "+", "`", w)
            ));
        } else {
            out.push(format!(
                "   {}   {}",
                hline(l.len(), "+", "+", "+", w),
                hline(r.len(), "+", "+", "+", w)
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
