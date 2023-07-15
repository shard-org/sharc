use crate::parser::{Data, Token};

pub fn compiler(tokens: Vec<Data>) -> Result<(), Vec<String>> {
    // TODO change the text field to a &str, prob by implementing a method
    // FIXME: .next() also doesnt work, might need to implement that too

    for data in tokens {
        match data.token {
            Token::Directive => match data.text {
                "use" => {
                    let filename = tokens.next().unwrap().text;
                }
            }

            _ => todo!(),
        }

    }

    todo!();
}
