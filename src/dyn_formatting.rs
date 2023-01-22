//! A module provides dynamic formatting like Python.

use crate::checker_error::{BoxedCheckerError, CheckerError, Stage};
use std::collections::HashMap;

/// Simple, dynamic, Python-styled string formatting (Only support `String`,
/// `{key}` patterns ).
///
/// Escape like `{{` or `}}`.
///
/// # Errors
///
/// 1. Raise `CheckerError::ArgFormattingKeyError` while the key in the brackets
///    is not found.
/// 2. Raise `CheckerError::ArgFormattingTokenError` while there is any
///    unmatched bracket (`{` or `}`)
pub fn dynamic_format(
    pattern: &str,
    dictionary: &HashMap<&str, &str>,
    stage: Stage,
) -> Result<String, BoxedCheckerError> {
    use CheckerError::*;
    use InnerError::*;
    _dynamic_format(pattern, dictionary).map_err(|e| {
        if let KeyError {
            pattern,
            key,
            dict_keys,
            pos,
        } = e
        {
            Box::new(ArgFormattingKeyError {
                stage,
                pattern,
                key,
                dict_keys,
                pos,
            })
        } else if let TokenError { pattern, desc, pos } = e {
            Box::new(ArgFormattingTokenError {
                stage,
                pattern,
                desc,
                pos,
            })
        } else {
            unreachable!();
        }
    })
}

fn _dynamic_format(pattern: &str, dictionary: &HashMap<&str, &str>) -> Result<String, InnerError> {
    if pattern.find('{') == None && pattern.find('}') == None {
        return Ok(pattern.to_string());
    }
    let chars: Vec<char> = pattern.chars().collect();
    let mut ans: String = String::with_capacity(pattern.len());
    let mut on_left_bracket = false;
    let mut last_left_bracket: usize = 0;
    let mut on_right_bracket = false;
    let mut last_right_bracket: usize = 0;
    let mut key = String::with_capacity(16);

    for (i, c) in chars.iter().enumerate() {
        match *c {
            '{' => {
                if on_left_bracket {
                    ans.push('{');
                    on_left_bracket = false;
                } else {
                    last_left_bracket = i;
                    on_left_bracket = true;
                }
            }
            '}' => {
                if on_right_bracket {
                    ans.push('}');
                    on_right_bracket = false;
                } else if on_left_bracket {
                    match dictionary.get(key.as_str()) {
                        Some(s) => ans.push_str(s),
                        None => {
                            return Err(InnerError::KeyError {
                                pattern: pattern.to_string(),
                                key,
                                dict_keys: dictionary.keys().map(|s| s.to_string()).collect(),
                                pos: last_left_bracket + 1,
                            })
                        }
                    };
                    key.clear();
                    on_left_bracket = false;
                } else {
                    on_right_bracket = true;
                    last_right_bracket = i;
                }
            }
            c => {
                if on_left_bracket {
                    key.push(c);
                } else {
                    ans.push(c);
                }
            }
        }
    }

    if on_left_bracket {
        return Err(InnerError::TokenError {
            pattern: pattern.to_string(),
            desc: "Unmatched token '{'".into(),
            pos: last_left_bracket,
        });
    }
    if on_right_bracket {
        return Err(InnerError::TokenError {
            pattern: pattern.to_string(),
            desc: "Unmatched token '}'".into(),
            pos: last_right_bracket,
        });
    }

    Ok(ans)
}

#[derive(Debug)]
enum InnerError {
    TokenError {
        pattern: String,
        desc: String,
        pos: usize,
    },
    KeyError {
        pattern: String,
        key: String,
        dict_keys: Vec<String>,
        pos: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::InnerError::*;
    use super::_dynamic_format;
    use std::collections::HashMap;

    macro_rules! dynamic_format {
        ($pattern: expr, $dict_list: expr) => {
            _dynamic_format(&$pattern.to_string(), &$dict_list.into())
        };
    }

    #[test]
    fn test_no_replace() {
        assert_eq!(dynamic_format!("", []).unwrap(), "".to_string());
        assert_eq!(
            dynamic_format!("abcdefg", []).unwrap(),
            "abcdefg".to_string()
        );
        assert_eq!(
            dynamic_format!("abc", [("abc", "")]).unwrap(),
            "abc".to_string()
        );
        assert_eq!(
            dynamic_format!("we-have", [("we", "")]).unwrap(),
            "we-have".to_string()
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(dynamic_format!("}}", []).unwrap(), "}".to_string());
        assert_eq!(
            dynamic_format!("{{ab}}", [("ab", "1")]).unwrap(),
            "{ab}".to_string()
        );
        assert_eq!(dynamic_format!("{{234", []).unwrap(), "{234".to_string());
        assert_eq!(dynamic_format!("{{{{a}}", []).unwrap(), "{{a}".to_string());
    }

    #[test]
    fn test_replace() {
        assert_eq!(
            dynamic_format!("{ab}", [("ab", "1")]).unwrap(),
            "1".to_string()
        );
        assert_eq!(
            dynamic_format!("1{a}32{a}4", [("a", "555"), ("b", "")]).unwrap(),
            "1555325554".to_string()
        );
        assert_eq!(
            dynamic_format!("{key1}-{key2}", [("key1", "0"), ("key2", "a")]).unwrap(),
            "0-a".to_string()
        );
    }

    #[test]
    fn test_mixed() {
        assert_eq!(
            dynamic_format!("{{{a}", [("a", "1")]).unwrap(),
            "{1".to_string()
        );
        assert_eq!(
            dynamic_format!("{{|{k}}}", [("k", "x123")]).unwrap(),
            "{|x123}".to_string()
        );
        assert_eq!(
            dynamic_format!("{{{key1}}}-}}}}{key2}", [("key1", "0"), ("key2", "a")]).unwrap(),
            "{0}-}}a".to_string()
        );
    }

    #[test]
    fn test_key_error() {
        match dynamic_format!("{abc}", [("abd", "1")]) {
            Err(KeyError {
                pattern,
                key,
                pos,
                dict_keys,
            }) => {
                assert_eq!(pattern.as_str(), "{abc}");
                assert_eq!(key, "abc");
                assert_eq!(dict_keys, vec!["abd"]);
                assert_eq!(pos, 1);
            }
            _ => unreachable!(),
        }
        match dynamic_format!("234{ac}{ab}", [("ac", "1"), ("aa", ".")]) {
            Err(KeyError { key, pos, .. }) => {
                assert_eq!(key, "ab");
                assert_eq!(pos, 8);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_token_error() {
        match dynamic_format!("{abc", [("abc", "1")]) {
            Err(TokenError { pattern, desc, pos }) => {
                assert_eq!(pattern.as_str(), "{abc");
                assert!(desc.find("'{'").is_some());
                assert_eq!(pos, 0);
            }
            _ => unreachable!(),
        }
        match dynamic_format!("{{a}}}324", []) {
            Err(TokenError { desc, pos, .. }) => {
                assert!(desc.find("'}'").is_some());
                assert_eq!(pos, 5);
            }
            _ => unreachable!(),
        }
    }
}
