use std::io::{Read, Write};

/// Represents an entry in the BNR errors and fixes JSON document.
#[allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BnrErrorFix {
    pub errorCode: u32,
    pub errorId: String,
    pub errorDescription: String,
    pub correctiveActionLevel1: String,
    pub correctiveActionLevel2: String,
    pub correctiveActionLevel3: String,
}

fn split_comment(comment: &mut String) {
    if let Some(nl) = comment.find('\n') {
        if nl == comment.len() - 1 {
            comment.replace_range(nl..nl + 1, "");
        } else {
            comment.replace_range(nl..nl + 1, "\n    ///   - ");
        }
    }
}

fn main() {
    let mut output = std::fs::File::create("error_and_fix.rs").expect("creating output file");
    let mut input =
        std::fs::File::open("globalBnrErrorsAndFixes.json").expect("opening input file");

    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("reading input to string");

    let space = " ";
    let tab = space.repeat(4);

    let error_fixes = match serde_json::from_str::<Vec<BnrErrorFix>>(content.as_str()) {
        Ok(e) => e,
        Err(err) => {
            println!("Error parsing JSON content: {err}");
            Vec::new()
        }
    };

    output
        .write_all(b"/// Represents BNR error codes.\n")
        .expect("writing rust header");
    output.write_all(b"///\n").expect("writing rust header");
    output.write_all(b"/// Documentation for each variant includes recommendations for resolving the error.\n").expect("writing rust header");
    output
        .write_all(b"#[allow(non_camel_case_types)]\n")
        .expect("writing rust header");
    output
        .write_all(b"#[repr(u32)]\n")
        .expect("writing rust header");
    output
        .write_all(b"#[derive(Clone, Copy, Debug, PartialEq)]\n")
        .expect("writing rust header");
    output
        .write_all(b"pub enum BnrError {\n")
        .expect("writing rust header");

    for ef in error_fixes.iter() {
        output
            .write_fmt(format_args!(
                "{tab}/// {}: {}\n",
                ef.errorId.trim_start_matches(' ').trim_end_matches(' '),
                ef.errorDescription
            ))
            .expect("writing ErrorFix docs");
        output
            .write_fmt(format_args!("{tab}///\n{tab}/// Corrective action(s):\n"))
            .expect("writing ErrorFix docs");

        if !ef.correctiveActionLevel1.is_empty() {
            let mut cor = ef.correctiveActionLevel1.clone();
            split_comment(&mut cor);
            output
                .write_fmt(format_args!("{tab}///\n{tab}/// - {cor}\n"))
                .expect("writing corrective action 1");
        }

        if !ef.correctiveActionLevel2.is_empty() {
            let mut cor = ef.correctiveActionLevel2.clone();
            split_comment(&mut cor);
            output
                .write_fmt(format_args!("{tab}///\n{tab}/// - {cor}\n"))
                .expect("writing corrective action 2");
        }

        if !ef.correctiveActionLevel3.is_empty() {
            let mut cor = ef.correctiveActionLevel3.clone();
            split_comment(&mut cor);
            output
                .write_fmt(format_args!("{tab}///\n{tab}/// - {cor}\n"))
                .expect("writing corrective action 3");
        }

        output
            .write_fmt(format_args!("{tab}{} = {},\n", ef.errorId, ef.errorCode))
            .expect("writing error definition");
    }

    output.write_all(b"}").expect("writing closing brace");

    output.write_all(b"\n\n").expect("");

    output
        .write_all(b"impl From<BnrError> for &'static str {\n")
        .expect("");
    output
        .write_fmt(format_args!("{tab}fn from(err: BnrError) -> Self {{\n"))
        .expect("");
    output
        .write_fmt(format_args!("{}match err {{\n", tab.repeat(2)))
        .expect("");

    for ef in error_fixes.iter() {
        let id = ef.errorId.as_str();
        let trim_id = id.trim_start_matches(' ').trim_end_matches(' ');
        let desc = if ef.errorDescription.is_empty() {
            String::new()
        } else {
            String::from(": ") + ef.errorDescription.as_str()
        };

        output
            .write_fmt(format_args!(
                "{}BnrError::{trim_id} => \"{trim_id}{desc}\",\n",
                tab.repeat(3)
            ))
            .expect("writing variant string conversion");
    }

    output
        .write_fmt(format_args!("{}}}\n", tab.repeat(2)))
        .expect("");
    output.write_fmt(format_args!("{tab}}}\n")).expect("");
    output.write_all(b"}\n\n").expect("");

    output
        .write_all(b"impl From<&BnrError> for &'static str {\n")
        .expect("");
    output
        .write_fmt(format_args!("{tab}fn from(err: &BnrError) -> Self {{\n"))
        .expect("");
    output
        .write_fmt(format_args!("{}(*err).into()\n", tab.repeat(2)))
        .expect("");
    output.write_fmt(format_args!("{tab}}}\n")).expect("");
    output.write_all(b"}\n\n").expect("");

    output
        .write_all(b"impl std::fmt::Display for BnrError {\n")
        .expect("");
    output
        .write_fmt(format_args!(
            "{tab}fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n"
        ))
        .expect("");
    output
        .write_fmt(format_args!(
            "{}write!(f, \"{{}}\", <&str>::from(self))\n",
            tab.repeat(2)
        ))
        .expect("");
    output.write_fmt(format_args!("{tab}}}\n")).expect("");
    output.write_all(b"}\n").expect("");
}
