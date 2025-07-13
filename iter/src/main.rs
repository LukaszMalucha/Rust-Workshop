
// fn print_elements(elements: &Vec<String>) {
//    for element in elements {
//         println!("{}", element);
//    } 

// }

// fn print_elements(elements: &Vec<String>) {
//     elements.iter().for_each(|el| println!("{}", el));
// }

fn print_elements(elements: &Vec<String>) {
    elements
    .iter().map(|el| format!("{} {}", el, el)).for_each(|el| println!("{} {}", el, el));
}


fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];


    print_elements(&colors);

    println!("{:#?}", colors);
}
