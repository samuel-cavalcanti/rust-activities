mod employee_control;
mod pig_latin;
mod statics_analysis;

fn main() {
    test_statics_analysis();

    test_pig_latin();

    test_employee_control();
}

fn test_statics_analysis() {
    let data_array = vec![20, 20, 20, 40, 25, 40];
    statics_analysis::statics_analysis::statics_analysis_from_array(data_array);
}

fn test_pig_latin() {
    let text = "a🧪: String";
    let pig_text = pig_latin::pig_latin::to_pig_latin(text.to_string());
    println!("{}", pig_text);

    let text = "🧪: String";
    let pig_text = pig_latin::pig_latin::to_pig_latin(text.to_string());
    println!("{}", pig_text);
}

fn test_employee_control() {
    employee_control::employee_control::start();
}