mod structs;

use std::collections::HashMap;
use std::sync::Mutex;
use structs::*;

type Check<T> = core::option::Option<T>;
static ENDING: &'static str = "\x1b[0m";

fn get_prefix(color: Color, is_background: bool) -> String {
    let mut temp = match color {
        Color::Black => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Yellow => 3,
        Color::Blue => 4,
        Color::Purple => 5,
        Color::Cyan => 6,
        Color::White => 7
    };
    if is_background { temp += 40 }
    else { temp += 30 }
    "\x1b[".to_owned() + &*temp.to_string() + "m"
}

fn get_style_prefix(style: Style) -> String {
    "\x1b[".to_owned() + &*match style {
        Style::Normal => 0,
        Style::Bold => 1,
        Style::Faded => 2,
        Style::Italic => 3,
        Style::Underlined => 4,
        Style::Flashing => 5,
        Style::Strikethrough => 6
    }.to_string() + "m"
}

fn background<S: AsRef<str>>(color: Color, text: S) -> String {
    let prefix = get_prefix(color, true);
    prefix + text.as_ref() + ENDING
}

fn font<S: AsRef<str>>(color: Color, text: S) -> String {
    let prefix = get_prefix(color, false);
    prefix + text.as_ref() + ENDING
}

fn style<S: AsRef<str>>(style: Style, text: S) -> String {
    let prefix = get_style_prefix(style);
    prefix + text.as_ref() + ENDING
}

fn font_background<S: AsRef<str>>(fontc: Color, back: Color, text: S) -> String {
    font(fontc, background(back, text))
}

fn style_font_background<S: AsRef<str>>(stylet: Style, fontc: Color, back: Color, text: S) -> String {
    style(stylet, font(fontc, background(back, text)))
}

/**
Replaces **sym** with a **color**
*/
fn paint_sym<S: AsRef<str>>(src: S, color: Color, sym: char) -> String {
    let mut res = StringBuilder::new();
    let prefix = get_prefix(color, true);
    for i in src.as_ref().chars() {
        if i == sym {
            res.add(prefix.clone());
            res.append(' ');
            res.add(ENDING);
            continue;
        }
        res.append(i);
    }
    res.build()
}

fn print_logo() {
    println!();
    println!("{}", font_background(Color::Black, Color::White, " -----  ----   \\   /  -----"));
    println!("{}", font_background(Color::Black, Color::White, "   |    |--     \\ /     |  "));
    println!("{}", font_background(Color::Black, Color::White, "   |    ----    / \\     |  "));
    println!();
}

fn print_help() {
    print_logo();
    println!("Author: TAFH-debug");
    println!("Beautiful text formatting utility. \nUsage: ");
    println!("      textf [<options>] <text>");
    println!("Options: ");
    println!("{}", "        --help - Shows this text.\n".to_owned() +
        "        -f, --font <color> - Set texts font color.\n" +
        "        -b, --background <color> - Set texts background color.\n" +
        "        -r, --random - Generate and show random image.\n" +
        "        -p, --print <color | style> - Prints info.\n");

}

fn get_color(text: String) -> Check<Color> {
    match &*text {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "purple" => Some(Color::Purple),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        _ => None
    }
}

fn get_style(text: String) -> Check<Style> {
    match &*text {
        "bold" => Some(Style::Bold),
        "italic" => Some(Style::Italic),
        "normal" => Some(Style::Normal),
        "faded" => Some(Style::Faded),
        "strikethrough" => Some(Style::Strikethrough),
        "underlined" => Some(Style::Underlined),
        "flashing" => Some(Style::Flashing),
        _ => None
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        print_help();
        return;
    }
    if args[1] == "--help" {
        print_help();
        return;
    }

    let mut options: HashMap<&str, fn(String) -> OptionType> = HashMap::new();

    fn style_flag(arg: String) -> OptionType {
        match get_style(arg) {
            Some(n) => OptionType::Style(n),
            None => {
                println!("{}", font(Color::Red, "Error: this style is not supported."));
                OptionType::Error
            },
        }
    }
    fn print_flag(arg: String) -> OptionType {
        match &*arg {
            "color" => {/*TODO*/},
            "style" => {/*TODO*/},
            _ => {
                println!("{}", font(Color::Red, "Error: undefined info type"));
                return OptionType::Error;
            },
        };
        OptionType::Print
    }
    fn font_flag(arg: String) -> OptionType {
        match get_color(arg) {
            Some(n) => OptionType::Font(n),
            None => {
                println!("{}", font(Color::Red, "Error: this color is not supported."));
                OptionType::Error
            },
        }
    }
    fn random_flag(arg: String) -> OptionType {
        todo!()
    }
    fn background_flag(arg: String) -> OptionType {
        match get_color(arg) {
            Some(n) => OptionType::Background(n),
            None => {
                println!("{}", font(Color::Red, "Error: this color is not supported."));
                OptionType::Error
            },
        }
    }

    options.insert("--print", print_flag);
    options.insert("-p", print_flag);
    options.insert("-s", style_flag);
    options.insert("-f", font_flag);
    options.insert("-b", background_flag);
    options.insert("-r", random_flag);
    options.insert("--style", style_flag);
    options.insert("--font", font_flag);
    options.insert("--background", background_flag);
    options.insert("--random", random_flag);

    let mut is_option = false;
    let mut uoptions = Vec::new();
    let mut prev_flag = String::new();
    let mut text = String::new();
    for i in args {
        if i.starts_with("-") {
            if !options.contains_key(i.as_str()) {
                println!("{}", font(Color::Red, "Error: invalid flag"));
                return;
            }
            if prev_flag != "" {
                uoptions.push(Option::new(prev_flag, "".to_string()));
            }
            prev_flag = i;
            is_option = true;
            continue;
        }
        if is_option {
            uoptions.push(Option::new(prev_flag.clone(), i));
            prev_flag = "".to_string();
            is_option = false;
            continue;
        }
        text = i;
    }
    let mut background_c = Color::Black;
    let mut font_c = Color::White;
    let mut style_t = Style::Normal;

    for i in uoptions {
        match (options.get(&*i.flag.clone()).unwrap())(i.value) {
            OptionType::Background(n) => background_c = n,
            OptionType::Font(n) => font_c = n,
            OptionType::Style(n) => style_t = n,
            OptionType::Print | OptionType::Random => (),
            OptionType::Error => return,
        }
    }
    println!("{}", style_font_background(style_t, font_c, background_c, text));
}