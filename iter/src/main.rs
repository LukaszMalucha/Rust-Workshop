use std::collections::LinkedList;
// fn print_elements(elements: &Vec<String>) {
//    for element in elements {
//         println!("{}", element);
//    } 

// }

// fn print_elements(elements: &Vec<String>) {
//     elements.iter().for_each(|el| println!("{}", el));
// }

fn print_elements(elements: &[String]) {
    elements
    .iter().map(|el| format!("{} {}", el, el)).for_each(|el| println!("{} {}", el, el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}



fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect::<Vec<_>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el))

}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements.iter().map(|el| el.chars().map(|c| c.to_string()).collect()).collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements.iter().find(|el| el.contains(search)).map_or(String::from(fallback), |el| el.to_string())
}


fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];


    // shorten_strings(&mut colors[1..3]);


    // println!("{:#?}", colors);

    // let uppercase = to_uppercase(&colors);
    // println!("{:#?}", uppercase);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);
    // let exploded = explode(&colors);
    // println!("Exploded: {:#?}", exploded);

    let found_color = find_color_or(&colors, "re", "Orange");
    println!("Exploded: {}", found_color);
}
