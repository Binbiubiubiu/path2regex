//! 2323

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub, private_in_public)]

pub mod parser;


pub(crate) fn escape_string<S>(input: S) -> anyhow::Result<String>
where
    S: AsRef<str>,
{
    let re = regex::Regex::new(r"([.+*?=^!:${}()\[\]|/\\])")?;
    let s = re.replace_all(input.as_ref(), "\\$1");
    Ok(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string(".+*?=^!:${}()[]|/\\3").unwrap(),r"\.\+\*\?\=\^\!\:\$\{\}\(\)\[\]\|\/\\3")
    }
}
