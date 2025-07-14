use text_io::read;

pub(crate) fn limit10(number: i32, denomination: i32) -> i32 {
    let value = number / denomination;

    if value > 10 {
        return 10;
    }

    return value;
}

pub(crate) fn read_char(default: char) -> char {
    let answer: String = read!("{}\n");
    let answer_char;

    if answer.is_empty() == false {
        answer_char = answer.chars().next().unwrap();
    } else {
        answer_char= default;
    }

    return answer_char;
}
