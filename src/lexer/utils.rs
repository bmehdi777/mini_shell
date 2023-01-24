pub fn is_authorize(input: u8) -> bool {
    input >= 33 && input <= 126 && (b'<' != input || b'>' != input || b'&' != input)
}
