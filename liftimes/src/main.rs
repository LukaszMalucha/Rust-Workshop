fn next_language(languages: &[String], current: &str) -> &str {
    let mut found = false;

    for lang in languages {
        if found {
          return lang;      
        }
        if lang == current {
            found = true
        }
    }

    languages.last().unwrap()

}


fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("js"),
    ]

    // let result = next_language(&langauges, "go");

    println!("{}", result);

}
