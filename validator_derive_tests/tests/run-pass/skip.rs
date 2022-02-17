use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(skip)]
    s: [u8; 11],
}

fn main() {}
