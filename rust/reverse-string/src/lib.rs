pub fn reverse(input: &str) -> String {
    let mut charcoll: Vec<char> = vec![];
    input.chars().for_each(|ichar| charcoll.push(ichar));
    let mut result = String::new();
    charcoll.reverse();
    charcoll.into_iter().for_each(|ichar| result.push(ichar));
    println!("{}", &result);
    result.to_string()
}
