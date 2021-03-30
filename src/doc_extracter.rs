pub mod markdown {
    use lazy_static::lazy_static;
    use regex::Regex;
    use regex::RegexBuilder;
    use std::collections::HashSet;
        
    pub fn extract_headings(text: &str) -> HashSet<String>{
        //lazy_static! macro compiles expression only once
        lazy_static! {
            //brackets create a capture grp
            static ref HEADINGS_REGEX: Regex = Regex::new(
                r"\## ([a-zA-Z][0-9a-zA-Z_].*)"
            ).unwrap();
        }
        HEADINGS_REGEX.captures_iter(text).map(|x| String::from(x.get(1).unwrap().as_str())).collect()
    }

    pub fn extract_bolded(text: &str) -> HashSet<String>{
        
        lazy_static! {
            //accept multiple regex expressions
            static ref BOLDEDWORDS_REGEX: Regex = Regex::new(
                r"\*\*([^\s].*?[\n]?.*?)\*\*|__([^\s]..*?[\n]?.*?)__"
            ).unwrap();
        }
        
        BOLDEDWORDS_REGEX.captures_iter(text)
            .map(|x| 
                x.get(0)
                .unwrap()
                .as_str()
                .replace("\n"," ")
                .replace("__","")
                .replace("**","")
            ).collect()
        
    }

    pub fn extract_italicised(text: &str) -> HashSet<String>{
        
        lazy_static! {
            //accept multiple regex expressions
            static ref ITALICISED_REGEX: Regex = Regex::new(
                r"[\s]\*([^\*\s]..*?[\n]?.*?)\*"
            ).unwrap();
        }
        
        ITALICISED_REGEX.captures_iter(text).map(|x| x.get(1).unwrap().as_str().replace("\n"," ")).collect()
        
    }

    pub fn extract_colored(text: &str) -> HashSet<String>{
        lazy_static! {
            static ref COLOREDWORDS_REGEX: Regex = Regex::new(
                r"<span style[^>]*>([^<>]+)<"
            ).unwrap();
        }
        //replace() to get rid of \n char
        COLOREDWORDS_REGEX.captures_iter(text).map(|x| x.get(1).unwrap().as_str().replace("\n"," ")).collect()
    }

    pub fn extract_blockquote(text: &str) -> HashSet<String>{
        
        lazy_static! {
            //accept multiple regex expressions
            static ref BLOCKQUOTE_REGEX: Regex = RegexBuilder::new(
                r"^(> ([a-zA-Z][0-9a-zA-Z_].*?[\n].*?)+.+)$|^(> ([a-zA-Z][0-9a-zA-Z_].*?).+)$"
            ).multi_line(true).build().unwrap();
            //regexBuilder allows for ^ and $ for multi-line expressions
        }
        
        BLOCKQUOTE_REGEX.captures_iter(text)
            .map(|x| 
                x.get(0)
                .unwrap()
                .as_str()
                .replace("\n"," ")
                .replace("> - ","")
                .replace(">","")
            ).collect()
        
    }
}

pub mod html {
    
}