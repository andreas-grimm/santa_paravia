pub(crate) fn limit10(number: i32, denomination: i32) -> i32 {
    let value = number / denomination;

    if value > 10 {
        return 10;
    }

    return value;
}
