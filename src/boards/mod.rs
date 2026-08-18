pub mod moonlander;

pub struct Board {
    pub macro_name: &'static str,
    pub render: fn(usize, &[String]) -> String,
}

pub static BOARDS: &[Board] = &[Board {
    macro_name: moonlander::MACRO,
    render: moonlander::render,
}];

pub fn find(macro_name: &str) -> Option<&'static Board> {
    BOARDS.iter().find(|b| b.macro_name == macro_name)
}
