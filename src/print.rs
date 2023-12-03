use bat::{Input, PagingMode, PrettyPrinter};

pub struct Pretty {}
impl Pretty {
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
    pub fn print_path(string: String) {
        PrettyPrinter::new()
            .input(Input::from_bytes(&string.as_bytes()))
            .header(true)
            .grid(true)
            .language("syslog")
            .theme("OneHalfDark")
            .print()
            .unwrap();
    }

    pub fn print_info(string: String) {
        PrettyPrinter::new()
            .input(Input::from_bytes(&string.as_bytes()))
            .header(false)
            .grid(false)
            .language("yaml")
            .theme("OneHalfDark")
            .print()
            .unwrap();
    }

    pub fn print_warning(string: String) {
        PrettyPrinter::new()
            .input(Input::from_bytes(&string.as_bytes()))
            .header(false)
            .grid(true)
            .language("log")
            .theme("OneHalfDark")
            .print()
            .unwrap();
    }

    pub fn print_error(string: String) {
        PrettyPrinter::new()
            .header(false)
            .grid(true)
            .line_numbers(false)
            .use_italics(true)
            .language("log")
            .theme("Monokai Extended Bright")
            .paging_mode(PagingMode::Never)
            .input(Input::from_bytes(&string.as_bytes()))
            .print()
            .unwrap();
    }
    pub fn print(string: String) {
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
}
