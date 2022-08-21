use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{append_highlighted_html_for_styled_line, IncludeBackground},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

use crate::blog_clusters::BlogClusters;
use crate::hlf_parser::{parse, HlfLhs, HlfRhs, Symbol};
use crate::shared::path_title;
use crate::shared::HTMLTemplate;

// 1. Retrieves the blogs into cluster
// 2. Parse the template file into HLF
// 3. Use the information in cluster to expand the HLF to get the webpage result
// For each kind of webpages the expand rules are different and hard-coded. The
// hard-coded rules could be wrote in files, which make this program data driven
// (But it's difficult and not practical because requirements are always ugly
// and hard to be described in a general way).

// Use a bnf-like thing is a fancier expression of html snippet provider
// while symbol in content means this symbol can be repeated

const LATEX_MARK: &[u8; 9] = b"lAtExhERE";
const LATEX_MARK_LEN: usize = LATEX_MARK.len();
const LATEX_TAG_BEGIN: &[u8; 19] = br#"<div class="latex">"#;
const LATEX_TAG_END: &[u8; 6] = b"</div>";

const ESCAPE_TABLE: [(&[u8], u8); 5] = [
    (b"&quot;", b'"'),
    (b"&amp;", b'&'),
    (b"&#39;", b'\''),
    (b"&lt;", b'<'),
    (b"&gt;", b'>'),
];

// With markdown as input, this function returns content with latex replaced by
// mark and array of latex extracted. Currently we don't need to distinguish
// block latex and inline latex, the metadata is stored with the string, here we
// kept the wrapping `$`, for inline latex number is `$` is 2, for block latex
// it's 4. And the second problem is if we should use Vec<u8> rather than String
// to represent each LaTeX part. The answer is no. We should use String to keep
// the information that this LaTeX part is valid UTF-8.
pub fn extract_latex(s: &str) -> (String, Vec<String>) {
    // Assume `$` pairs in a line as latex code fence.
    let s = s.as_bytes();
    let mut new_s = Vec::with_capacity(s.len());
    let mut latexes = Vec::new();

    let mut state = 0;

    let mut ptr = 0;
    /* Here is a simple automaton for latex extraction
     * State Transition Graph:
     * L: Line break: `\r`, `\n`
     * $: Dollar sign
     * o: Other chars
     * If `o` is not set for specific state, we can ensure any other chars won't change this state.
     * ```
     * +-+--$->+-+     +-+
     * |2|     |0|<-L--|5|
     * +-+--L->+-+     +-+
     * A      A|AA      A
     * |      //||      |
     * |     // $\      o
     * o    L/   \L     |
     * |   //     \\    |
     * |  //       \\   |
     * | //         \\  |
     * ||$           \\ |
     * |||            \\|
     * ||V            |||
     * +-+     +-+     +-+
     * |1|--$->|3|--$->|4|
     * +-+     +-+     +-+
     * ```
     */
    for (i, &byte) in s.iter().enumerate() {
        match state {
            0 => match byte {
                b'$' => {
                    state = 1;
                    new_s.extend(&s[ptr..i]);
                    ptr = i;
                }
                _ => (),
            },
            1 => match byte {
                b'$' => state = 3,
                b'\r' | b'\n' => state = 0,
                _ => state = 2,
            },
            2 => {
                match byte {
                    b'$' => {
                        state = 0;
                        new_s.extend(LATEX_MARK);
                        latexes.push(unsafe { String::from_utf8_unchecked(s[ptr..=i].to_vec()) });
                        ptr = i + 1;
                    }
                    // This is the error state, here we don't recover, instead
                    // we only escape to next line. Why we need the `\r` check?
                    // My special thank to Apple and Microsoft -_-
                    b'\r' | b'\n' => state = 0,
                    _ => (),
                }
            }
            3 => match byte {
                b'$' => state = 4,
                _ => (),
            },
            4 => match byte {
                b'$' => {
                    state = 0;
                    new_s.extend(LATEX_MARK);
                    latexes.push(unsafe { String::from_utf8_unchecked(s[ptr..=i].to_vec()) });
                    ptr = i + 1;
                }
                b'\r' | b'\n' => state = 0,
                _ => state = 5,
            },
            // 5 is the error state, we only want to meet a line break then we will forget the error
            5 => {
                match byte {
                    // Here we recover from the error state
                    b'\r' | b'\n' => state = 0,
                    _ => (),
                }
            }
            _ => unreachable!(),
        }
    }
    new_s.extend(&s[ptr..]);

    (unsafe { String::from_utf8_unchecked(new_s) }, latexes)
}

// Replace marks in string given with latexes given. If latexes given more than
// marks in string, this function returns None.
pub fn insert_latex(s: &str, latexes: &Vec<String>) -> Option<String> {
    let s = s.as_bytes();
    let mut latexes_iter = 0;
    let mut begin = 0;
    let mut result = Vec::with_capacity(s.len());
    let mut i = 0;
    let i_max = s.len() - LATEX_MARK_LEN;
    while i < i_max {
        if &s[i..i + LATEX_MARK_LEN] == LATEX_MARK {
            result.extend(&s[begin..i]);
            result.extend(LATEX_TAG_BEGIN);
            result.extend(html_escape(&latexes[latexes_iter]).as_bytes());
            result.extend(LATEX_TAG_END);
            latexes_iter += 1;
            begin = i + LATEX_MARK_LEN;
            i = begin;
            if latexes_iter >= latexes.len() {
                break;
            }
        } else {
            i += 1;
        }
    }
    if latexes_iter < latexes.len() {
        None
    } else {
        result.extend(&s[begin..]);
        Some(unsafe { String::from_utf8_unchecked(result) })
    }
}

// This is used for unescape html
pub fn html_unescape<T: AsRef<str>>(s: T) -> String {
    let s = s.as_ref().as_bytes();

    let (begin, mut result) = (0..s.len()).fold(
        (0, Vec::with_capacity(s.len())),
        |(mut begin, mut result), i| {
            // unescape process
            ESCAPE_TABLE
                .iter()
                .find_map(|(before, after)| {
                    s.get(i..i + before.len()).and_then(|range| {
                        if &range == before {
                            Some((before.len(), after))
                        } else {
                            None
                        }
                    })
                })
                .map(|(offset, after)| {
                    result.extend(&s[begin..i]);
                    result.push(*after);
                    begin = i + offset;
                });
            (begin, result)
        },
    );
    // Append the tail
    result.extend(&s[begin..]);
    // The input is &str so we can ensure there is no surprise.
    unsafe { String::from_utf8_unchecked(result) }
}

pub fn html_escape<T: AsRef<str>>(s: T) -> String {
    let s = s.as_ref().as_bytes();

    let (begin, mut result) = s.iter().enumerate().fold(
        (0, Vec::with_capacity(s.len())),
        |(mut begin, mut result), (i, byte)| {
            ESCAPE_TABLE
                .iter()
                .find_map(
                    |(before, after)| {
                        if byte == after {
                            Some(before)
                        } else {
                            None
                        }
                    },
                )
                .map(|&before| {
                    result.extend(&s[begin..i]);
                    result.extend(before);
                    begin = i + 1;
                });
            (begin, result)
        },
    );
    result.extend(&s[begin..]);
    unsafe { String::from_utf8_unchecked(result) }
}

// Transform several frequently used markdown code annotation to file extension
pub fn lang2ext(lang: &str) -> &str {
    match lang {
        // Syntect have no ebnf syntax highlighting support :-/
        "cpp" | "c++" | "cxx" => "cpp",
        "rust" => "rs",
        "pascal" => "pas",
        "ebnf" | "" => "txt",
        _ => lang,
    }
}

pub fn highlight_code(lang: &str, code: &str) -> String {
    static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
    static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| ThemeSet::load_defaults());

    let syntax = SYNTAX_SET
        .find_syntax_by_extension(lang2ext(lang))
        .expect(&format!("Unknown language: {}!", lang));
    let ref theme = THEME_SET.themes["base16-ocean.light"];
    let mut highlighter = HighlightLines::new(syntax, theme);

    let code_unesc = html_unescape(&code);
    let mut code_highlight = String::with_capacity(code_unesc.len() * 2);

    for line in LinesWithEndings::from(&code_unesc) {
        let regions = highlighter.highlight(line, &SYNTAX_SET);
        append_highlighted_html_for_styled_line(
            &regions,
            IncludeBackground::No,
            &mut code_highlight,
        );
    }

    code_highlight
}

pub struct BlogTemplate {
    hlfs: HashMap<HlfLhs, HlfRhs>,
}

impl HTMLTemplate for BlogTemplate {
    fn load(template_raw: &str) -> Result<Self, String> {
        let hlfs_vec = match parse(&template_raw) {
            Some(x) => x,
            None => return Err("template parse failed".to_string()),
        };
        let mut hlfs = HashMap::new();
        for i in hlfs_vec.iter() {
            hlfs.insert(i.lhs.clone(), i.rhs.clone());
        }
        Ok(Self { hlfs: hlfs })
    }

    fn fill(&self, cluster: &BlogClusters) -> Vec<(String, String)> {
        let mut results = Vec::new();

        // We have the knowledge of blog template's structure
        let main_rhs = self
            .hlfs
            .get("main")
            .expect("there should be a main symbol in blog template.");
        let tags_rhs = match main_rhs.get(1).unwrap() {
            Symbol::N(x) => self
                .hlfs
                .get(x)
                .expect(&format!("\"{}\" symbol not found.", x)),
            _ => panic!(),
        };
        let tag_rhs = match tags_rhs.get(1).unwrap() {
            Symbol::N(x) => self
                .hlfs
                .get(x)
                .expect(&format!("\"{}\" symbol not found.", x)),
            _ => panic!(),
        };
        assert_eq!(main_rhs.len(), 3);
        assert_eq!(tags_rhs.len(), 3);
        assert_eq!(tag_rhs.len(), 1);

        let blogs = cluster.get_blogs();
        for blog in blogs {
            let mut result = String::new();
            match main_rhs.get(0).unwrap() {
                Symbol::T(x) => {
                    // 1. Markdown to html
                    // 2. Retrieve code blocks in html.
                    // 3. Do syntax highlighting on unescaped code blocks
                    //    according to code annotation. (code may contains
                    //    some characters will be escaped to fit into html)

                    // This solution is inspired by author of comrak:
                    // https://github.com/kivikakk/comrak/issues/129. But
                    // actually a better solution is extracting code blocks
                    // before converting markdown to html and insert the
                    // highlighted code after it. This is how we process latex
                    // blocks, but I come up with it before I finish the code
                    // highlighting part :-P. It works anyway....

                    let options = ComrakOptions {
                        // Enable frequently used github markdown extensions
                        extension: ComrakExtensionOptions {
                            tasklist: true,
                            table: true,
                            strikethrough: true,
                            ..Default::default()
                        },
                        render: ComrakRenderOptions {
                            github_pre_lang: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    let (content, latexes) = extract_latex(&blog.content);
                    let content = markdown_to_html(&content, &options);
                    let raw_html = x
                        .replace("_slot_of_blog_title", &blog.title)
                        .replace("_slot_of_blog_day", &blog.day.to_string())
                        .replace("_slot_of_blog_month", &blog.month.to_string())
                        .replace("_slot_of_blog_year", &blog.year.to_string())
                        .replace("_slot_of_blog_preview", &blog.preview)
                        .replace("_slot_of_blog_content", &content);
                    let raw_html = match insert_latex(&raw_html, &latexes) {
                        Some(x) => x,
                        None => panic!("LaTeX insertion error!"),
                    };
                    // Assume latex never overlaps with or contained by code.
                    static RE: Lazy<Regex> = Lazy::new(|| {
                        Regex::new(r#"<pre lang="([^"]*)"><code>([^<]*)</code></pre>"#).unwrap()
                    });
                    let mut begin = 0;
                    for cap in RE.captures_iter(&raw_html) {
                        let lang = cap.get(1).unwrap().as_str();
                        let code = cap.get(2).unwrap().as_str();
                        let ref code_highlight = highlight_code(lang, code);
                        let range = cap.get(0).unwrap().range();
                        let end = range.start;
                        result.push_str(&raw_html[begin..end]);
                        result.push_str(r#"<pre lang=""#);
                        result.push_str(lang);
                        result.push_str(r#""><code>"#);
                        result.push_str(code_highlight);
                        result.push_str(r#"</code></pre>"#);
                        begin = range.end;
                    }
                    result.push_str(&raw_html[begin..]);
                }
                _ => panic!(),
            };
            match tags_rhs.get(0).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            };
            // add multiple tag names
            for tag_handle in blog.tags.iter() {
                match tag_rhs.get(0).unwrap() {
                    Symbol::T(x) => {
                        let tag = cluster.get_tag(*tag_handle).unwrap();
                        result.push_str(&x.replace("_slot_of_tag_name", &tag.name));
                    }
                    _ => panic!(),
                };
            }
            match tags_rhs.get(2).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            };
            match main_rhs.get(2).unwrap() {
                Symbol::T(x) => result.push_str(x),
                _ => panic!(),
            }
            results.push((format!("{}{}", &path_title(&blog.title), ".html"), result));
        }
        results
    }
}

#[cfg(test)]
mod template_tests {
    use super::*;

    #[test]
    fn test_html_unescape() {
        assert_eq!(html_unescape("emm"), "emm");

        assert_eq!(html_unescape("&quot;"), "\"");
        assert_eq!(html_unescape("&amp;"), "&");
        assert_eq!(html_unescape("&#39;"), "\'");
        assert_eq!(html_unescape("&lt;"), "<");
        assert_eq!(html_unescape("&gt;"), ">");

        assert_eq!(html_unescape("&emm"), "&emm");
        assert_eq!(html_unescape("&quot"), "&quot");
        assert_eq!(html_unescape("&qu&lt;"), "&qu<");
        assert_eq!(html_unescape("&qu&lt"), "&qu&lt");
        assert_eq!(html_unescape("&quot;&lt;"), "\"<");
        assert_eq!(html_unescape("&quot;&lt"), "\"&lt");
        assert_eq!(html_unescape("&lt;&quot;&quot;&gt;"), "<\"\">");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("emm"), "emm");
        assert_eq!("&quot;", html_escape("\""));
        assert_eq!("&amp;", html_escape("&"));
        assert_eq!("&#39;", html_escape("\'"));
        assert_eq!("&lt;", html_escape("<"));
        assert_eq!("&gt;", html_escape(">"));

        assert_eq!("&amp;emm", html_escape("&emm"));
        assert_eq!("&amp;quot", html_escape("&quot"));
        assert_eq!("&amp;qu&lt;", html_escape("&qu<"));
        assert_eq!("&amp;qu&amp;lt", html_escape("&qu&lt"));
        assert_eq!("&quot;&lt;", html_escape("\"<"));
        assert_eq!("&quot;&amp;lt", html_escape("\"&lt"));
        assert_eq!("&lt;&quot;&quot;&gt;", html_escape("<\"\">"));
    }

    #[test]
    fn test_html_escape_and_unescape() {
        let chaos = r#"
        $%^Y&UIafjnh%^&*(OGFTY^&*IOL<KO{}?L:"KJYT<><<<>>"""KK'''
        'L';'''"''"'""<><><>GFDER$%^&*()*&^%$%YH^T&*UIOJHVYFT^&Y
        *IOUYTE@#!@#$%^&*((~!@#$%^&*()(*^%~`1234567897^%$#@!@#$%
        ^&*148964865}"?>:{}"?><LP{}"?><KJHGBNL;oijk,./'][p;.,mnb
        vcxsrtyjkghmnabsdjf])))
        "#;

        assert_eq!(html_unescape(&html_escape(&chaos)), chaos);
    }

    #[test]
    fn test_inline_latex_extraction() {
        let s = "
            hi $I'm latex0$ alice
            hi $I'm latex1$ bob
            hi $I'm not latex
            hi $I'm latex2$ $I'm not latex
            hi $I'm latex3$ alice hi $I'm latex4$ bob
        ";
        let (_, latexes) = extract_latex(s);
        assert_eq!(
            latexes,
            [
                "$I'm latex0$",
                "$I'm latex1$",
                "$I'm latex2$",
                "$I'm latex3$",
                "$I'm latex4$",
            ]
        );
    }

    #[test]
    fn test_block_latex_extraction() {
        let s = "
            hi $$I'm latex0$$ alice
            hi $$I'm latex1 bob
            hahah$$
            hi $$I'm a failed latex$
            hi $$I'm latex2$$ $I'm not latex
            hi $$I'm latex3$$ alice hi $$I'm latex4$$ bob
        ";
        let (_, latexes) = extract_latex(s);
        assert_eq!(
            latexes,
            [
                "$$I'm latex0$$",
                "$$I'm latex1 bob
            hahah$$",
                "$$I'm latex2$$",
                "$$I'm latex3$$",
                "$$I'm latex4$$",
            ]
        );
    }

    #[test]
    fn test_inline_latex_insertion() {
        let mark: &str = unsafe { &String::from_utf8_unchecked(LATEX_MARK.to_vec()) };
        let begin: &str = unsafe { &String::from_utf8_unchecked(LATEX_TAG_BEGIN.to_vec()) };
        let end: &str = unsafe { &String::from_utf8_unchecked(LATEX_TAG_END.to_vec()) };
        let s = ["a", mark, "b", mark, "c"].join("");
        let latexes = vec![String::from("$Alice$"), String::from("$Bob$")];
        let s = insert_latex(&s, &latexes);
        assert_eq!(
            s,
            Some(
                [
                    "a",
                    begin,
                    &latexes[0],
                    end,
                    "b",
                    begin,
                    &latexes[1],
                    end,
                    "c"
                ]
                .join("")
            )
        );
    }

    #[test]
    fn test_block_latex_insertion() {
        let mark: &str = unsafe { &String::from_utf8_unchecked(LATEX_MARK.to_vec()) };
        let begin: &str = unsafe { &String::from_utf8_unchecked(LATEX_TAG_BEGIN.to_vec()) };
        let end: &str = unsafe { &String::from_utf8_unchecked(LATEX_TAG_END.to_vec()) };
        let s = ["a", mark, "b", mark, "c"].join("");
        let latexes = vec![String::from("$$Ali\nce$$"), String::from("$$Bob$$")];
        let s = insert_latex(&s, &latexes);
        assert_eq!(
            s,
            Some(
                [
                    "a",
                    begin,
                    &latexes[0],
                    end,
                    "b",
                    begin,
                    &latexes[1],
                    end,
                    "c"
                ]
                .join("")
            )
        );
    }
}
