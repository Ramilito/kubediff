use bat::{Input, PagingMode, PrettyPrinter};
use std::process::{Command, Stdio};

// pub fn print_themes() {
//     let printer = PrettyPrinter::new();
//     println!("Themes:");
//     for theme in printer.themes() {
//         println!("- {}", theme);
//     }
// }

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
        .highlight(line!() as usize)
        .language("diff")
        .theme("gruvbox-dark")
        .paging_mode(PagingMode::Never)
        .print()
        .unwrap();
}

pub fn get_target() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let target = format!(
        "{}/workspace/toca-boca/toca-days-platform/Services/travel-service/k8s/local",
        home_dir.display()
    );
    target
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
