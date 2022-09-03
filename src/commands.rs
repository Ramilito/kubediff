use bat::{Input, PagingMode, PrettyPrinter};
use std::process::{Child, Command, Stdio};

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

pub fn get_diff() -> Child {
    Command::new("kubectl")
        .env("KUBECTL_EXTERNAL_DIFF", format!("{}", get_script()))
        .arg("diff")
        .arg("-f")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
}

pub fn get_build(target: &String) -> String {
    let output = Command::new("kustomize")
        .arg("build")
        .arg(target)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    String::from_utf8(output.stdout.to_owned()).unwrap()
}

pub fn get_script() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("diff.sh");
    path.to_str().unwrap().to_string()
}
