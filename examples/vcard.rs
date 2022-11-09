use vcard4::Vcard;

pub fn main() {
    let mut card = Vcard::new(String::from("John Doe"));
    card.nickname.push(String::from("Johnny").into());
    print!("{}", card);
}
