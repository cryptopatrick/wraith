/// Parser used to try and tokenize communications made during 
/// T1E1, Transmission 1 Element (1) Identification.
/// Please not that the parser can fail, and therefore returns a Result<>.
/// 
/// Sentenses in T1E1 will typically have the following format:
/// ADDRESSEE DELIMITER CALLSIGN_PHRASE
/// "Sledgehammer, this is Merlin 1"

#[derive(PartialEq, Clone, Debug)]
pub enum Delimiter {
    Space,
    Comma,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Addressee(String),
    Delimiter,
    Callsign(String),
}

impl FromStr for Expr {
    type Err = String; 
    fn from_str(_s: &str) -> Result<Expr, String> {
        unimplemented! {}
    }
}

// A phrase that can be parsed into     tokens that are allowed during the T1E1.
pub fn indentification_phrase(
    a: Expr::Addressee, 
    d: Expr::Delimiter,
    // TODO: this is missing handling for pronoun phrases, like "this is"
    c: Expr::Callsign,
) -> Expr {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t1e1(){
        let e: Expr = "Sledgehammer, this is Merlin 1".parse().unwrap();
        // assert_eq!()
        unimplemented!();
    }
}