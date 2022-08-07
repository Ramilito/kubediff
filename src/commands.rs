use bat::{Input, PagingMode, PrettyPrinter};
use std::process::{Command, Stdio};

pub fn diff() {
    let home_dir = dirs::home_dir().unwrap();
    let target = format!(
        "{}/workspace/toca-boca/toca-days-platform/Services/kiali/k8s/dev",
        home_dir.display()
    );

    let output = Command::new("kubectl")
        .env("KUBECTL_EXTERNAL_DIFF", format!("{}/workspace/mine/kubediff/src/diff.sh", home_dir.display()))
        .arg("diff")
        .arg("-k")
        .arg(target)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    let string = String::from_utf8(output.stdout.to_owned()).unwrap();
    println!("{}", string);

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
        .theme("1337")
        .paging_mode(PagingMode::Never)
        .print()
        .unwrap();

}
