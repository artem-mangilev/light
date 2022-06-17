pub fn render(html: String) {
    let mut in_angle = false;

    for char in html.chars() {
        if char == '<' {
            in_angle = true;
        } else if char == '>' {
            in_angle = false;
        } else if !in_angle {
            print!("{}", char)

        }
    }
}