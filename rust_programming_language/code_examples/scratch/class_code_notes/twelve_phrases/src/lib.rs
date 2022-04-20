pub mod greetings {
    pub mod english;
    pub mod french;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/* Documentation notes
*/


#[test]
fn english_greeting_correct() {
    assert_eq!("hello", greetings::english::hello());
}