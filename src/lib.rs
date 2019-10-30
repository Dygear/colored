#![allow(unused_imports, dead_code, unused_parens)]

//!Coloring terminal so simple, you already know how to do it !
//!
//!    use colored::Colorize;
//!
//!    "this is blue".blue();
//!    "this is red".red();
//!    "this is red on blue".red().on_blue();
//!    "this is also red on blue".on_blue().red();
//!    "you can also make bold comments".bold();
//!    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
//!    "or change advice. This is red".yellow().blue().red();
//!    "or clear things up. This is default color and style".red().bold().clear();
//!    "purple and magenta are the same".purple().magenta();
//!    "bright colors are also allowed".bright_blue().on_bright_white();
//!    "you can specify color by string".color("blue").on_color("red");
//!    "and so are normal and clear".normal().clear();
//!    String::from("this also works!").green().bold();
//!    format!("{:30}", "format works as expected. This will be padded".blue());
//!    format!("{:.3}", "and this will be green but truncated to 3 chars".green());
//!
//!
//! See [the `Colorize` trait](./trait.Colorize.html) for all the methods.
//!

#[macro_use]
extern crate lazy_static;
#[cfg(windows)]
extern crate winconsole;

#[cfg(test)]
extern crate rspec;

mod color;
pub mod control;
mod style;

pub use color::*;

use std::convert::From;
use std::fmt;
use std::ops::Deref;
use std::string::String;

/// A string that may have color and/or style applied to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
    style: style::Style,
}

/// The trait that enables something to be given color.
///
/// You can use `colored` effectively simply by importing this trait
/// and then using its methods on `String` and `&str`.
pub trait Colorize {
    // Font Colors
    fn black(self) -> ColoredString;
    fn red(self) -> ColoredString;
    fn green(self) -> ColoredString;
    fn yellow(self) -> ColoredString;
    fn blue(self) -> ColoredString;
    fn magenta(self) -> ColoredString;
    fn purple(self) -> ColoredString;
    fn cyan(self) -> ColoredString;
    fn white(self) -> ColoredString;
    fn bright_black(self) -> ColoredString;
    fn bright_red(self) -> ColoredString;
    fn bright_green(self) -> ColoredString;
    fn bright_yellow(self) -> ColoredString;
    fn bright_blue(self) -> ColoredString;
    fn bright_magenta(self) -> ColoredString;
    fn bright_purple(self) -> ColoredString;
    fn bright_cyan(self) -> ColoredString;
    fn bright_white(self) -> ColoredString;
    fn color<S: Into<Color>>(self, color: S) -> ColoredString;
    // Background Colors
    fn on_black(self) -> ColoredString;
    fn on_red(self) -> ColoredString;
    fn on_green(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
    fn on_blue(self) -> ColoredString;
    fn on_magenta(self) -> ColoredString;
    fn on_purple(self) -> ColoredString;
    fn on_cyan(self) -> ColoredString;
    fn on_white(self) -> ColoredString;
    fn on_bright_black(self) -> ColoredString;
    fn on_bright_red(self) -> ColoredString;
    fn on_bright_green(self) -> ColoredString;
    fn on_bright_yellow(self) -> ColoredString;
    fn on_bright_blue(self) -> ColoredString;
    fn on_bright_magenta(self) -> ColoredString;
    fn on_bright_purple(self) -> ColoredString;
    fn on_bright_cyan(self) -> ColoredString;
    fn on_bright_white(self) -> ColoredString;
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
    // Styles
    fn clear(self) -> ColoredString;
    fn normal(self) -> ColoredString;
    fn bold(self) -> ColoredString;
    fn dimmed(self) -> ColoredString;
    fn italic(self) -> ColoredString;
    fn underline(self) -> ColoredString;
    fn blink(self) -> ColoredString;
    /// Historical name of `Colorize::reversed`. May be removed in a future version. Please use
    /// `Colorize::reversed` instead
    fn reverse(self) -> ColoredString;
    /// This should be preferred to `Colorize::reverse`.
    fn reversed(self) -> ColoredString;
    fn hidden(self) -> ColoredString;
    fn strikethrough(self) -> ColoredString;
}

impl ColoredString {
    pub fn is_plain(&self) -> bool {
        (self.bgcolor.is_none() && self.fgcolor.is_none() && self.style == style::CLEAR)
    }

    #[cfg(not(feature = "no-color"))]
    fn has_colors(&self) -> bool {
        use control;

        control::SHOULD_COLORIZE.should_colorize()
    }

    #[cfg(feature = "no-color")]
    fn has_colors(&self) -> bool {
        false
    }

    fn compute_style(&self) -> String {
        if !self.has_colors() || self.is_plain() {
            return String::new();
        }

        let mut res = String::from("\x1B[");
        let mut has_wrote = if self.style != style::CLEAR {
            res.push_str(&self.style.to_str());
            true
        } else {
            false
        };

        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(fgcolor.to_fg_str());
        }

        res.push('m');
        res
    }

    fn escape_inner_reset_sequences(&self) -> String {
        if !self.has_colors() || self.is_plain() {
            return self.input.clone();
        }

        // TODO: BoyScoutRule
        let reset = "\x1B[0m";
        let style = self.compute_style();
        let matches: Vec<usize> = self
            .input
            .match_indices(reset)
            .map(|(idx, _)| idx)
            .collect();

        let mut input = self.input.clone();
        input.reserve(matches.len() * style.len());

        for (idx_in_matches, offset) in matches.into_iter().enumerate() {
            // shift the offset to the end of the reset sequence and take in account
            // the number of matches we have escaped (which shift the index to insert)
            let mut offset = offset + reset.len() + idx_in_matches * style.len();

            for cchar in style.chars() {
                input.insert(offset, cchar);
                offset += 1;
            }
        }

        input
    }
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
            style: style::CLEAR,
        }
    }
}

impl Deref for ColoredString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.input
    }
}

impl<'a> From<&'a str> for ColoredString {
    fn from(s: &'a str) -> Self {
        ColoredString {
            input: String::from(s),
            ..ColoredString::default()
        }
    }
}

macro_rules! def_color {
    ($side:ident: $name: ident => $color: path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                $side: Some($color), .. self
            }
        }
    };
}

macro_rules! def_style {
    ($name: ident, $value: path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                style: style::Style::from_both(self.style, $value),
                .. self
            }
        }
    };
}

impl Colorize for ColoredString {
    def_color!(fgcolor: black => Color::Black);
    fn red(self) -> ColoredString {
        self.color(Color::Red)
    }
    def_color!(fgcolor: green => Color::Green);
    def_color!(fgcolor: yellow => Color::Yellow);
    def_color!(fgcolor: blue => Color::Blue);
    def_color!(fgcolor: magenta => Color::Magenta);
    def_color!(fgcolor: purple => Color::Magenta);
    def_color!(fgcolor: cyan => Color::Cyan);
    def_color!(fgcolor: white => Color::White);
    def_color!(fgcolor: bright_black => Color::BrightBlack);
    fn bright_red(self) -> ColoredString {
        self.color(Color::BrightRed)
    }
    def_color!(fgcolor: bright_green => Color::BrightGreen);
    def_color!(fgcolor: bright_yellow => Color::BrightYellow);
    def_color!(fgcolor: bright_blue => Color::BrightBlue);
    def_color!(fgcolor: bright_magenta => Color::BrightMagenta);
    def_color!(fgcolor: bright_purple => Color::BrightMagenta);
    def_color!(fgcolor: bright_cyan => Color::BrightCyan);
    def_color!(fgcolor: bright_white => Color::BrightWhite);

    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            ..self
        }
    }

    def_color!(bgcolor: on_black => Color::Black);
    fn on_red(self) -> ColoredString {
        ColoredString {
            bgcolor: Some(Color::Red),
            ..self
        }
    }
    def_color!(bgcolor: on_green => Color::Green);
    def_color!(bgcolor: on_yellow => Color::Yellow);
    def_color!(bgcolor: on_blue => Color::Blue);
    def_color!(bgcolor: on_magenta => Color::Magenta);
    def_color!(bgcolor: on_purple => Color::Magenta);
    def_color!(bgcolor: on_cyan => Color::Cyan);
    def_color!(bgcolor: on_white => Color::White);

    def_color!(bgcolor: on_bright_black => Color::BrightBlack);
    fn on_bright_red(self) -> ColoredString {
        ColoredString {
            bgcolor: Some(Color::BrightRed),
            ..self
        }
    }
    def_color!(bgcolor: on_bright_green => Color::BrightGreen);
    def_color!(bgcolor: on_bright_yellow => Color::BrightYellow);
    def_color!(bgcolor: on_bright_blue => Color::BrightBlue);
    def_color!(bgcolor: on_bright_magenta => Color::BrightMagenta);
    def_color!(bgcolor: on_bright_purple => Color::BrightMagenta);
    def_color!(bgcolor: on_bright_cyan => Color::BrightCyan);
    def_color!(bgcolor: on_bright_white => Color::BrightWhite);

    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            ..self
        }
    }

    fn clear(self) -> ColoredString {
        ColoredString {
            input: self.input,
            ..ColoredString::default()
        }
    }
    fn normal(self) -> ColoredString {
        self.clear()
    }
    def_style!(bold, style::Styles::Bold);
    def_style!(dimmed, style::Styles::Dimmed);
    def_style!(italic, style::Styles::Italic);
    def_style!(underline, style::Styles::Underline);
    def_style!(blink, style::Styles::Blink);
    def_style!(reverse, style::Styles::Reversed);
    def_style!(reversed, style::Styles::Reversed);
    def_style!(hidden, style::Styles::Hidden);
    def_style!(strikethrough, style::Styles::Strikethrough);
}

macro_rules! def_str_color {
    ($side:ident: $name: ident => $color: path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                input: String::from(self),
                $side: Some($color),
                .. ColoredString::default()
            }
        }
    }
}

macro_rules! def_str_style {
    ($name:ident, $style:path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                input: String::from(self),
                style: style::Style::new($style),
                .. ColoredString::default()
            }
        }
    }
}

impl<'a> Colorize for &'a str {
    def_str_color!(fgcolor: black => Color::Black);
    fn red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: Some(Color::Red),
            ..ColoredString::default()
        }
    }
    def_str_color!(fgcolor: green => Color::Green);
    def_str_color!(fgcolor: yellow => Color::Yellow);
    def_str_color!(fgcolor: blue => Color::Blue);
    def_str_color!(fgcolor: magenta => Color::Magenta);
    def_str_color!(fgcolor: purple => Color::Magenta);
    def_str_color!(fgcolor: cyan => Color::Cyan);
    def_str_color!(fgcolor: white => Color::White);

    def_str_color!(fgcolor: bright_black => Color::BrightBlack);
    fn bright_red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: Some(Color::BrightRed),
            ..ColoredString::default()
        }
    }
    def_str_color!(fgcolor: bright_green => Color::BrightGreen);
    def_str_color!(fgcolor: bright_yellow => Color::BrightYellow);
    def_str_color!(fgcolor: bright_blue => Color::BrightBlue);
    def_str_color!(fgcolor: bright_magenta => Color::BrightMagenta);
    def_str_color!(fgcolor: bright_purple => Color::BrightMagenta);
    def_str_color!(fgcolor: bright_cyan => Color::BrightCyan);
    def_str_color!(fgcolor: bright_white => Color::BrightWhite);

    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    def_str_color!(bgcolor: on_black => Color::Black);
    fn on_red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            bgcolor: Some(Color::Red),
            ..ColoredString::default()
        }
    }
    def_str_color!(bgcolor: on_green => Color::Green);
    def_str_color!(bgcolor: on_yellow => Color::Yellow);
    def_str_color!(bgcolor: on_blue => Color::Blue);
    def_str_color!(bgcolor: on_magenta => Color::Magenta);
    def_str_color!(bgcolor: on_purple => Color::Magenta);
    def_str_color!(bgcolor: on_cyan => Color::Cyan);
    def_str_color!(bgcolor: on_white => Color::White);

    def_str_color!(bgcolor: on_bright_black => Color::BrightBlack);
    fn on_bright_red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            bgcolor: Some(Color::BrightRed),
            ..ColoredString::default()
        }
    }
    def_str_color!(bgcolor: on_bright_green => Color::BrightGreen);
    def_str_color!(bgcolor: on_bright_yellow => Color::BrightYellow);
    def_str_color!(bgcolor: on_bright_blue => Color::BrightBlue);
    def_str_color!(bgcolor: on_bright_magenta => Color::BrightMagenta);
    def_str_color!(bgcolor: on_bright_purple => Color::BrightMagenta);
    def_str_color!(bgcolor: on_bright_cyan => Color::BrightCyan);
    def_str_color!(bgcolor: on_bright_white => Color::BrightWhite);

    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    fn clear(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            style: style::CLEAR,
            ..ColoredString::default()
        }
    }
    fn normal(self) -> ColoredString {
        self.clear()
    }
    def_str_style!(bold, style::Styles::Bold);
    def_str_style!(dimmed, style::Styles::Dimmed);
    def_str_style!(italic, style::Styles::Italic);
    def_str_style!(underline, style::Styles::Underline);
    def_str_style!(blink, style::Styles::Blink);
    def_str_style!(reverse, style::Styles::Reversed);
    def_str_style!(reversed, style::Styles::Reversed);
    def_str_style!(hidden, style::Styles::Hidden);
    def_str_style!(strikethrough, style::Styles::Strikethrough);
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.has_colors() || self.is_plain() {
            return (<String as fmt::Display>::fmt(&self.input, f));
        }

        // XXX: see tests. Useful when nesting colored strings
        let escaped_input = self.escape_inner_reset_sequences();

        try!(f.write_str(&self.compute_style()));
        try!(<String as fmt::Display>::fmt(&escaped_input, f));
        try!(f.write_str("\x1B[0m"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting() {
        // respect the formatting. Escape sequence add some padding so >= 40
        assert!(format!("{:40}", "".blue()).len() >= 40);
        // both should be truncated to 1 char before coloring
        assert_eq!(
            format!("{:1.1}", "toto".blue()).len(),
            format!("{:1.1}", "1".blue()).len()
        )
    }

    #[test]
    fn it_works() {
        let toto = "toto";
        println!("{}", toto.red());
        println!("{}", String::from(toto).red());
        println!("{}", toto.blue());

        println!("blue style ****");
        println!("{}", toto.bold());
        println!("{}", "yeah ! Red bold !".red().bold());
        println!("{}", "yeah ! Yellow bold !".bold().yellow());
        println!("{}", toto.bold().blue());
        println!("{}", toto.blue().bold());
        println!("{}", toto.blue().bold().underline());
        println!("{}", toto.blue().italic());
        println!("******");
        println!("test clearing");
        println!("{}", "red cleared".red().clear());
        println!("{}", "bold cyan cleared".bold().cyan().clear());
        println!("******");
        println!("Bg tests");
        println!("{}", toto.green().on_blue());
        println!("{}", toto.on_magenta().yellow());
        println!("{}", toto.purple().on_yellow());
        println!("{}", toto.magenta().on_white());
        println!("{}", toto.cyan().on_green());
        println!("{}", toto.black().on_white());
        println!("******");
        println!("{}", toto.green());
        println!("{}", toto.yellow());
        println!("{}", toto.purple());
        println!("{}", toto.magenta());
        println!("{}", toto.cyan());
        println!("{}", toto.white());
        println!("{}", toto.white().red().blue().green());
        // uncomment to see term output
        // assert!(false)
    }

    #[test]
    fn compute_style_empty_string() {
        assert_eq!("", "".clear().compute_style());
    }

    #[test]
    fn compute_style_simple_fg_blue() {
        let blue = "\x1B[34m";

        assert_eq!(blue, "".blue().compute_style());
    }

    #[test]
    fn compute_style_simple_bg_blue() {
        let on_blue = "\x1B[44m";

        assert_eq!(on_blue, "".on_blue().compute_style());
    }

    #[test]
    fn compute_style_blue_on_blue() {
        let blue_on_blue = "\x1B[44;34m";

        assert_eq!(blue_on_blue, "".blue().on_blue().compute_style());
    }

    #[test]
    fn compute_style_simple_fg_bright_blue() {
        let blue = "\x1B[94m";

        assert_eq!(blue, "".bright_blue().compute_style());
    }

    #[test]
    fn compute_style_simple_bg_bright_blue() {
        let on_blue = "\x1B[104m";

        assert_eq!(on_blue, "".on_bright_blue().compute_style());
    }

    #[test]
    fn compute_style_bright_blue_on_bright_blue() {
        let blue_on_blue = "\x1B[104;94m";

        assert_eq!(
            blue_on_blue,
            "".bright_blue().on_bright_blue().compute_style()
        );
    }

    #[test]
    fn compute_style_simple_bold() {
        let bold = "\x1B[1m";

        assert_eq!(bold, "".bold().compute_style());
    }

    #[test]
    fn compute_style_blue_bold() {
        let blue_bold = "\x1B[1;34m";

        assert_eq!(blue_bold, "".blue().bold().compute_style());
    }

    #[test]
    fn compute_style_blue_bold_on_blue() {
        let blue_bold_on_blue = "\x1B[1;44;34m";

        assert_eq!(
            blue_bold_on_blue,
            "".blue().bold().on_blue().compute_style()
        );
    }

    #[test]
    fn escape_reset_sequence_spec_should_do_nothing_on_empty_strings() {
        let style = ColoredString::default();
        let expected = String::new();

        let output = style.escape_inner_reset_sequences();

        assert_eq!(expected, output);
    }

    #[test]
    fn escape_reset_sequence_spec_should_do_nothing_on_string_with_no_reset() {
        let style = ColoredString {
            input: String::from("hello world !"),
            ..ColoredString::default()
        };

        let expected = String::from("hello world !");
        let output = style.escape_inner_reset_sequences();

        assert_eq!(expected, output);
    }

    #[test]
    fn escape_reset_sequence_spec_should_replace_inner_reset_sequence_with_current_style() {
        let input = format!("start {} end", String::from("hello world !").red());
        let style = input.blue();

        let output = style.escape_inner_reset_sequences();
        let blue = "\x1B[34m";
        let red = "\x1B[31m";
        let reset = "\x1B[0m";
        let expected = format!("start {}hello world !{}{} end", red, reset, blue);
        assert_eq!(expected, output);
    }

    #[test]
    fn escape_reset_sequence_spec_should_replace_multiple_inner_reset_sequences_with_current_style()
    {
        let italic_str = String::from("yo").italic();
        let input = format!(
            "start 1:{} 2:{} 3:{} end",
            italic_str, italic_str, italic_str
        );
        let style = input.blue();

        let output = style.escape_inner_reset_sequences();
        let blue = "\x1B[34m";
        let italic = "\x1B[3m";
        let reset = "\x1B[0m";
        let expected = format!(
            "start 1:{}yo{}{} 2:{}yo{}{} 3:{}yo{}{} end",
            italic, reset, blue, italic, reset, blue, italic, reset, blue
        );

        println!("first: {}\nsecond: {}", expected, output);

        assert_eq!(expected, output);
    }

    #[test]
    fn color_fn() {
        assert_eq!("blue".blue(), "blue".color("blue"))
    }

    #[test]
    fn on_color_fn() {
        assert_eq!("blue".on_blue(), "blue".on_color("blue"))
    }

    #[test]
    fn bright_color_fn() {
        assert_eq!("blue".bright_blue(), "blue".color("bright blue"))
    }

    #[test]
    fn on_bright_color_fn() {
        assert_eq!("blue".on_bright_blue(), "blue".on_color("bright blue"))
    }
}
