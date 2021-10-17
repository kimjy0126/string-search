use regex::Regex;

#[derive(Debug)]
pub struct Searcher {
    does_replace_initial: bool,
    does_ignore_whitespace: bool,
    is_case_sensitive: bool,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher { does_replace_initial: false,
                       does_ignore_whitespace: false,
                       is_case_sensitive: true }
    }

    pub fn replace_initial(mut self, does_replace_initial: bool) -> Searcher {
        self.does_replace_initial = does_replace_initial;
        self
    }

    pub fn ignore_whitespace(mut self, does_ignore_whitespace: bool) -> Searcher {
        self.does_ignore_whitespace = does_ignore_whitespace;
        self
    }

    pub fn case_sensitive(mut self, is_case_sensitive: bool) -> Searcher {
        self.is_case_sensitive = is_case_sensitive;
        self
    }

    pub fn search<'a>(&self, needle: &str, haystack: &'a Vec<String>) -> Vec<&'a str> {
        let mut needle: String = needle.to_string();

        if self.does_replace_initial {
            needle = Searcher::process_initial(&needle);
        }

        if self.does_ignore_whitespace {
            needle = Searcher::remove_whitespace(&needle);
        }

        if !self.is_case_sensitive {
            needle = needle.to_lowercase();
        }

        let re_needle = Regex::new(&needle).unwrap();
        let mut result: Vec<&str> = vec![];

        for hay in haystack.iter() {
            let mut processed_hay: String = hay.to_string();
            if self.does_ignore_whitespace {
                processed_hay = Searcher::remove_whitespace(&processed_hay);
            }
            if !self.is_case_sensitive {
                processed_hay = processed_hay.to_lowercase();
            }
            
            if re_needle.is_match(&processed_hay) {
                result.push(&hay);
            }
        }

        result
    }

    fn process_initial(s: &str) -> String {
        let re = Regex::new(r"ㄱ").unwrap();
        let s = &re.replace_all(s, "(ㄱ|[가-깋])");
        let re = Regex::new(r"ㄲ").unwrap();
        let s = &re.replace_all(s, "(ㄲ|[까-낗])");
        let re = Regex::new(r"ㄴ").unwrap();
        let s = &re.replace_all(s, "(ㄴ|[나-닣])");
        let re = Regex::new(r"ㄷ").unwrap();
        let s = &re.replace_all(s, "(ㄷ|[다-딯])");
        let re = Regex::new(r"ㄸ").unwrap();
        let s = &re.replace_all(&s, "(ㄸ|[따-띻])");
        let re = Regex::new(r"ㄹ").unwrap();
        let s = &re.replace_all(&s, "(ㄹ|[라-맇])");
        let re = Regex::new(r"ㅁ").unwrap();
        let s = &re.replace_all(&s, "(ㅁ|[마-밓])");
        let re = Regex::new(r"ㅂ").unwrap();
        let s = &re.replace_all(&s, "(ㅂ|[바-빟])");
        let re = Regex::new(r"ㅃ").unwrap();
        let s = &re.replace_all(&s, "(ㅃ|[빠-삫])");
        let re = Regex::new(r"ㅅ").unwrap();
        let s = &re.replace_all(&s, "(ㅅ|[사-싷])");
        let re = Regex::new(r"ㅆ").unwrap();
        let s = &re.replace_all(&s, "(ㅆ|[싸-앃])");
        let re = Regex::new(r"ㅇ").unwrap();
        let s = &re.replace_all(&s, "(ㅇ|[아-잏])");
        let re = Regex::new(r"ㅈ").unwrap();
        let s = &re.replace_all(&s, "(ㅈ|[자-짛])");
        let re = Regex::new(r"ㅉ").unwrap();
        let s = &re.replace_all(&s, "(ㅉ|[짜-찧])");
        let re = Regex::new(r"ㅊ").unwrap();
        let s = &re.replace_all(&s, "(ㅊ|[차-칳])");
        let re = Regex::new(r"ㅋ").unwrap();
        let s = &re.replace_all(&s, "(ㅋ|[카-킿])");
        let re = Regex::new(r"ㅌ").unwrap();
        let s = &re.replace_all(&s, "(ㅌ|[타-팋])");
        let re = Regex::new(r"ㅍ").unwrap();
        let s = &re.replace_all(&s, "(ㅍ|[파-핗])");
        let re = Regex::new(r"ㅎ").unwrap();
        let s = &re.replace_all(&s, "(ㅎ|[하-힣])");
    
        s.to_string()
    }

    fn remove_whitespace(s: &str) -> String {
        let re_remove_whitespace = Regex::new("( |\t|\n)").unwrap();

        re_remove_whitespace.replace_all(s, "").into_owned()
    }
}