//! A few functions to customize the backgroud colour, foreground colour, and style of the terminal
//! output. 

/// foreground colour in RGB format
pub fn fg(r: u8, g: u8, b: u8) {
    print!("\x1b[38;2;{};{};{};1m",r,g,b);
}

/// background colour in RGB format
pub fn bg(r: u8, g: u8, b: u8) {
    print!("\x1b[48;2;{};{};{};1m",r,g,b);
}

/// style
///
/// * 1 → bold
/// * 2 → dimmed
/// * 3 → italic
/// * 4 → underline
/// * 5 → blink
/// * 7 → reversed
/// * 8 → hidden
/// * 9 → strikethrough
pub fn style(style: u8) {
    print!("\x1b[{};1m",style);
}

/// reset the style
pub fn reset() {
    print!("\x1b[0m");
}

/// add colour to a string
pub fn add_fg(s: String, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{};1m{}\x1b[0m",r,g,b,s)
}

/// add background colour to a string
pub fn add_bg(s: String, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[40;2;{};{};{};1m{}\x1b[0m",r,g,b,s)
}

/// add style to a string
pub fn add_style(s: String, style: u8) -> String {
    if style <= 9 {
        return format!("\x1b[{};1m{}\x1b[0m",style,s);
    } else {
        return s;
    }
}
