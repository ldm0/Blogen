/*
Design:
Use comment chunk of html for convenient HLF preview

<!--symbol-->main<!--symbol-->
<!--content-->content<!--content-->

document := symbol content document | epsilon
content := content symbol content | epsilon
*/
// This parser is strict, error when possible.

use std::str::Chars;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Symbol {
    N(String),
    T(String),
}

pub type HlfLhs = String;
pub type HlfRhs = Vec<Symbol>; // Ns and Ts

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HLF {
    pub lhs: HlfLhs,
    pub rhs: HlfRhs,
}

impl HLF {
    pub fn new() -> Self {
        HLF {
            lhs: String::new(),
            rhs: Vec::<Symbol>::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum HlfType {
    Symbol,
    Content,
}

// Check if input prefix with pattern
fn match_str<'a, 'b>(input: &Chars<'a>, pattern: &Chars<'b>) -> Option<Chars<'a>> {
    let mut input = input.clone();
    let mut pattern = pattern.clone();
    loop {
        let patternch = match pattern.next() {
            Some(x) => x,
            None => break,
        };
        if let Some(inputch) = input.next() {
            if inputch != patternch {    // if not equal
                return None;
            }
        } else {    // if x to the end
            return None;
        }
    }
    Some(input)
}

fn match_type_begin<'a>(input: &Chars<'a>) -> Option<Chars<'a>> {
    const TYPE_BEGIN: &str = "<!--";
    match_str(input, &mut TYPE_BEGIN.chars())
}

fn match_type_end<'a>(input: &Chars<'a>) -> Option<Chars<'a>> {
    const TYPE_END: &str = "-->";
    match_str(input, &mut TYPE_END.chars())
}

fn match_type<'a>(input: &Chars<'a>) -> Option<(Chars<'a>, HlfType)> {
    let mut input_it = input.clone(); 
    let mut enclose = String::new();
    if let Some(it) = match_type_begin(&input_it) {
        input_it = it;
    } else {
        return None;
    }
    loop {
        if let Some(it) = match_type_end(&input_it) {
            input_it = it;
            return match enclose.trim() {
                "symbol" => Some((input_it, HlfType::Symbol)),
                "content" => Some((input_it, HlfType::Content)),
                _ => None,
            };
        } else {
            match input_it.next() {
                Some(x) => enclose.push(x),
                None => return None,
            }
        }
    }
}

pub fn parse(input: &str) -> Option<Vec<HLF>> {
    let mut input = input.chars();

    let mut result: Vec<HLF> = Vec::new();
    //let mut symbol_table: HashSet<String> = ::new();

    // currently should get which part of a HLF
    let mut get_right: bool = false;
    let mut insymbol: bool = false;
    let mut incontent: bool = false;

    let mut tmp_hlf: HLF = HLF::new();
    let mut tmp_str = String::new();

    loop {
        if get_right {  // get right side of HLF
            if incontent {  // get right side and in content chunk
                if insymbol {   // get right side and in content's symbol part
                    if let Some((it, typ)) = match_type(&input) {
                        match typ {
                            HlfType::Symbol => {
                                input = it;
                                insymbol = false;
                                // symbol should be trimmed
                                tmp_hlf.rhs.push(Symbol::N(tmp_str.trim().to_string()));
                                tmp_str.clear();
                                //println!("symbol in content close");
                            }
                            HlfType::Content => {
                                // no content in symbol segment
                                return None;
                            }
                        }
                    } else {
                        match input.next() {
                            Some(ch) => tmp_str.push(ch),
                            None => return None,
                        }
                    }
                } else {        // get right side and in content's non-symbol part
                    if let Some((it, typ)) = match_type(&input) {
                        match typ {
                            HlfType::Symbol => {
                                input = it;
                                insymbol = true;
                                tmp_hlf.rhs.push(Symbol::T(tmp_str.clone()));
                                tmp_str.clear();
                                //println!("get a symbol in content");
                            }
                            HlfType::Content => {
                                // content ends
                                input = it;
                                incontent = false;
                                get_right = false;
                                tmp_hlf.rhs.push(Symbol::T(tmp_str.clone()));
                                tmp_str.clear();
                                result.push(tmp_hlf);
                                tmp_hlf = HLF::new();
                                //println!("content close");
                            }
                        }
                    } else {
                        match input.next() {
                            // append to content
                            Some(ch) => tmp_str.push(ch),
                            None => return None,
                        }
                    }
                }
            } else { // get right side and not in content
                if let Some((it, typ)) = match_type(&input) {
                    match typ {
                        HlfType::Content => {
                            input = it;
                            incontent = true;
                            //println!("get a content");
                        }
                        HlfType::Symbol => {
                            return None;
                        }
                    }
                } else {
                    // ignore 
                    match input.next() {
                        Some(_) => (),
                        None => return None,
                    }
                }
            }
        } else { // get left side of HLF
            if insymbol { //left side and in symbol
                if let Some((it, typ)) = match_type(&input){
                    match typ {
                        HlfType::Symbol => {
                            input = it;
                            insymbol = false;
                            get_right = true;
                            tmp_hlf.lhs = tmp_str.trim().to_string();
                            tmp_str.clear();
                            //println!("symbol close");
                        }
                        HlfType::Content => {
                            // content are not permitted in symbol
                            return None;
                        }
                    }
                } else {
                    // append symbol
                    match input.next() {
                        Some(ch) => tmp_str.push(ch),
                        None => return None,
                    }
                }
            } else {    // left side and not in symbol
                if let Some((it, typ)) = match_type(&input) {
                    match typ {
                        HlfType::Symbol => {
                            input = it;
                            insymbol = true;
                            //println!("get a symbol");
                        }
                        HlfType::Content => {
                            // content are not permitted in symbol
                            return None;
                        }
                    }
                } else {
                    // ignore
                    match input.next() {
                        Some(_) => (),
                        None => return Some(result),
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod hlf_parser_tests {
    use super::*;
    macro_rules! is_none {
        ($option: expr) => {
            match $option {
                Some(_) => false,
                None => true,
            }
        };
    }

    #[test]
    fn test_match_str() {
        macro_rules! can_match {
            ($input: expr, $pattern: expr) => {
                assert!(!is_none!(match_str(&$input.chars(), &$pattern.chars())));
            };
        }
        macro_rules! cannot_match {
            ($input: expr, $pattern: expr) => {
                assert!(is_none!(match_str(&$input.chars(), &$pattern.chars())));
            };
        }
        can_match!("a", "a");
        can_match!("aaa", "aaa");
        can_match!("Hello😁world Yeah!", "Hello😁world");
        can_match!("😁world Yeah!", "😁world");
        cannot_match!("a😁world Yeah!", "😁world");
        cannot_match!("😁worl😁world Yeah!", "😁world");
        cannot_match!("world", "worldemm");
        cannot_match!("world", "aworld");
    }

    #[test]
    fn test_match_type_begin() {
        macro_rules! can_match_begin {
            ($input: expr) => {
                assert!(!is_none!(match_type_begin(&$input.chars())));
            }
        }
        macro_rules! cannot_match_begin {
            ($input: expr) => {
                assert!(is_none!(match_type_begin(&$input.chars())));
            }
        }
        can_match_begin!("<!--");
        can_match_begin!("<!----");
        cannot_match_begin!("<!-");
        cannot_match_begin!("<!-<!--");
        cannot_match_begin!("😒<!--");
    }

    #[test]
    fn test_match_type_end() {
        macro_rules! can_match_end {
            ($input: expr) => {
                assert!(!is_none!(match_type_end(&$input.chars())));
            }
        }
        macro_rules! cannot_match_end {
            ($input: expr) => {
                assert!(is_none!(match_type_end(&$input.chars())));
            }
        }
        can_match_end!("-->");
        can_match_end!("-->--");
        cannot_match_end!("--->");
        cannot_match_end!("<!-->");
        cannot_match_end!("<!----");
        cannot_match_end!("<!--");
    }

    #[test]
    fn test_match_type() {
        let (_, typ) = match_type(&"<!--symbol-->".chars()).unwrap();
        assert_eq!(HlfType::Symbol, typ);
        let (_, typ) = match_type(&"<!--content-->".chars()).unwrap();
        assert_eq!(HlfType::Content, typ);
        if let None = match_type(&"<!--hahaha-->".chars()) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_basic() {
        let input = "<!--symbol-->a<!--symbol--><!--content-->b<!--content-->";
        let result = parse(input).unwrap();
        assert_eq!(result.len(), 1);
        let result: &HLF = &result[0];
        assert_eq!(result.lhs, String::from("a"));
        assert_eq!(result.rhs, vec![Symbol::T(String::from("b"))]);
    }

    #[test]
    fn test_parse_discrete_symbol() {
        let input = "<!--symbol-->this is the symbol<!--symbol--><!--content-->this is the content<!--content-->";
        let result = parse(input).unwrap();
        assert_eq!(result.len(), 1);
        let result: &HLF = &result[0];
        assert_eq!(result.lhs, String::from("this is the symbol"));
        assert_eq!(result.rhs, vec![Symbol::T(String::from("this is the content"))]);
    }

    #[test]
    fn test_parse_symbol_type_trim() {
        // test if can be trimmed
        let input = "<!-- symbol -->this is the symbol<!-- symbol --><!-- content -->this is the content<!-- content -->";
        let result = parse(input).unwrap();
        assert_eq!(result.len(), 1);
        let result: &HLF = &result[0];
        assert_eq!(result.lhs, String::from("this is the symbol"));
        assert_eq!(result.rhs, vec![Symbol::T(String::from("this is the content"))]);
    }

    #[test]
    fn test_parse_symbol_name_trim_content_not_trim() {
        let input = "<!--symbol-->     symbol_name_with_padding     <!--symbol--><!--content--> content with padding <!--content-->";
        let result = parse(input).unwrap();
        assert_eq!(result.len(), 1);
        let result: &HLF = &result[0];
        assert_eq!(result.lhs, String::from("symbol_name_with_padding"));
        assert_eq!(result.rhs, vec![Symbol::T(String::from(" content with padding "))]);
    }

    #[test]
    fn test_parse_symbol_name_is_conmment() {
        let input = "<!--symbol-->     <!--this is a comment--> <!--symbol--><!--content--> content with padding <!--content-->";
        let symbol = "<!--this is a comment-->";
        let result = parse(input).unwrap();
        assert_eq!(result[0].lhs, symbol.to_string());
    }

    #[test]
    fn test_parse_dirty() {
        let input = "
        emmm 
        hah hsdfa
        <!--symbol-->     symbol_name_with_padding     <!--symbol-->
        <!--content-->
        a little content with padding 
        :w
        tesst 
        helloworld
        <!--content-->
        emmm
        <!--symbol-->     symbol_name_with_padding     <!--symbol-->
        <!--content-->
        a little content with padding 
        :w
        tesst 
        helloworld
        <!--content-->
        emm
        ";
        let symbol = "symbol_name_with_padding";
        let content = "
        a little content with padding 
        :w
        tesst 
        helloworld
        ";
        let result = parse(input).unwrap();
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].rhs.len(), 1);
        assert_eq!(result[1].rhs.len(), 1);

        assert_eq!(result[0].lhs, symbol.to_string());
        assert_eq!(result[1].lhs, symbol.to_string());
        assert_eq!(result[0].rhs[0], Symbol::T(content.to_string()));
        assert_eq!(result[1].rhs[0], Symbol::T(content.to_string()));
    }

    #[test]
    fn test_parse_symbol_in_content() {
        let input = "
        <!--symbol--> this is the symbol <!--symbol-->
        <!--content-->
        symbol front guard
            <!--symbol-->
                symbol in content
            <!--symbol-->
        symbol back guard
        <!--content-->
        ";
        let result = parse(input).unwrap();
        assert_eq!(result.len(), 1);
        let result = &result[0];
        assert_eq!(result.rhs.len(), 3);
    }

    #[test]
    #[should_panic]
    fn test_parse_no_content() {
        //let 
        let input = "<!--symbol--> the symbol <!--symbol-->";
        parse(input).unwrap();
    }
}