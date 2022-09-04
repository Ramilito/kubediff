use bat::{Input, PagingMode, PrettyPrinter};

#[allow(dead_code)]
pub fn print_themes() {
    let printer = PrettyPrinter::new();

    println!("Syntaxes:");
    for syntax in printer.syntaxes() {
        println!("- {} ({})", syntax.name, syntax.file_extensions.join(", "));
    }

    println!();

    println!("Themes:");
    for theme in printer.themes() {
        println!("- {}", theme);
    }
}

pub fn pretty_print_path(string: String) {
    PrettyPrinter::new()
        .input(Input::from_bytes(&string.as_bytes()))
        .header(true)
        .grid(true)
        .language("syslog")
        .theme("OneHalfDark")
        .print()
        .unwrap();
}

pub fn pretty_print_info(string: String) {
    PrettyPrinter::new()
        .input(Input::from_bytes(&string.as_bytes()))
        .header(false)
        .grid(false)
        .language("yaml")
        .theme("OneHalfDark")
        .print()
        .unwrap();
}

pub fn pretty_print(string: String) {
    PrettyPrinter::new()
        .input(
            Input::from_bytes(&string.as_bytes())
                .name("diff.yaml")
                .kind("File"),
        )
        .header(true)
        .grid(true)
        .line_numbers(true)
        .use_italics(true)
        .language("diff")
        .theme("gruvbox-dark")
        .paging_mode(PagingMode::Never)
        .print()
        .unwrap();
}

