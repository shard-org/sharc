
#[derive(Debug)]
pub struct Verb {
    pub verb: &'static str, 
    pub args: Vec<&'static str>,
}

pub struct ParsedVerb {
    name:  String,
    exec:  String,
    args:  Vec<String>,
    stdin: String,
}

pub trait VerbExt {
    fn parse(input: &str) -> Self;
}

impl VerbExt for Vec<ParsedVerb> {
    fn parse(input: &str) -> Self {
        for line in input.lines() {
            
        }
        todo!()
    }

    // fn find(verb: &str) {
    // }
    //
    // fn execute() {
    // }
}

// :verb run /bin/sh {
//    echo "test"
// }

