use rust_analyzer_use_lib_bug_report::Example;

fn main() {
    let example = Example {
        x: 32,
        y: "Hello".to_string(),
    };

    println!("Print example: {:?}", example);
}
