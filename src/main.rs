fn main() {
    let text = "Error handling is the process of handling the possibility of failure. For example, failing to read a file and then continuing to use that bad input would clearly be problematic. Noticing and explicitly managing those errors saves the rest of the program from various pitfalls. There are various ways to deal with errors in Rust, which are described in the following subchapters. They all have more or less subtle differences and different use cases. As a rule of thumb: An explicit panic is mainly useful for tests and dealing with unrecoverable errors. For prototyping it can be useful, for example when dealing with functions that haven't been implemented yet, but in those cases the more descriptive unimplemented is better. In tests panic is a reasonable way to explicitly fail. The Option type is for when a value is optional or when the lack of a value is not an error condition. For example the parent of a directory - / and C: don't have one. When dealing with Options, unwrap is fine for prototyping and cases where it's absolutely certain that there is guaranteed to be a value. However expect is more useful since it lets you specify an error message in case something goes wrong anyway.";
    println!("{}", wrapper(text, 20));
}

fn wrapper(text: &str, column: usize) -> &str {

    return ""
}

fn wrapper_helper(text: &str, column: usize) -> (&str, Option<&str>) {
    let mut text_divided: (&str, Option<&str>) = ("", None);

    if text.len() <= column {
        return (text, None)
    }

    let mut position_text: i32 = (column as i32) - 1;
    loop {
        if position_text == -1{
            text_divided.0 = text;
            text_divided.1 = None;
            break;
        }

        let position_text_usize = position_text as usize;
        let letter = text.chars().nth(position_text_usize);
        if letter == Some(' ') {
            text_divided.0 = &text[..position_text_usize];
            text_divided.1 = Some(&text[position_text_usize + 1..]);
            break;
        }

        position_text -= 1;
    }

    return text_divided;
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_parameter_text_length_greater_than_one() {
        let text = wrapper("", 1);
        assert!(text.len() > 0, "El texto debe tener al menos un caracter")
    }

    // #[test]
    // fn test_parameter_column_greater_than_one() {
    //     let text = wrapper("este texto va a ser mas largo", 0);
    //     assert!(text.len() > 0, "El texto debe tener al menos un caracter")
    // }

}