// - Install wasmpack
// - Run wasm-pack build --release --target web

use khi::html::{write_html, PreprocessorError as HtmlPreprocessorError};
use khi::parse::{error_to_string, parse_expression_document};
use khi::tex::{PreprocessorError as TexPreprocessorError, write_tex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check_well_formed(str: &str) -> Result<(), String> {
    match parse_expression_document(str) {
        Ok(_) => Ok(()),
        Err(e) => Err(error_to_string(&e)),
    }
}

#[wasm_bindgen]
pub fn preprocess_latex(str: &str) -> Result<String, String> {
    let doc = match parse_expression_document(str) {
        Ok(v) => v,
        Err(e) => return Err(error_to_string(&e)),
    };
    let tex = match write_tex(&doc) {
        Ok(v) => v,
        Err(e) => return Err(match e {
            TexPreprocessorError::IllegalTable(p) => format!("Illegal sequence found at {}:{}.", p.line, p.column),
            TexPreprocessorError::IllegalDictionary(p) => format!("Illegal dictionary found at {}:{}.", p.line, p.column),
            TexPreprocessorError::ZeroTable(p) => format!("Table cannot be empty at {}:{}.", p.line, p.column),
            TexPreprocessorError::UnknownCommand(p, c) => format!("Unknown command {} found at {}:{}.", &c, p.line, p.column),
            TexPreprocessorError::MissingOptionalArgument(p) => format!("Missing optional argument at {}:{}.", p.line, p.column),
        }),
    };
    Ok(tex)
}

#[wasm_bindgen]
pub fn preprocess_html(str: &str) -> Result<String, String> {
    let doc = match parse_expression_document(str) {
        Ok(v) => v,
        Err(e) => return Err(error_to_string(&e)),
    };
    let tex = match write_html(&doc) {
        Ok(v) => v,
        Err(e) => return Err(match e {
            HtmlPreprocessorError::IllegalSequence(p) => format!("Illegal sequence found at {}:{}.", p.line, p.column),
            HtmlPreprocessorError::IllegalDictionary(p) => format!("Illegal dictionary found at {}:{}.", p.line, p.column),
            HtmlPreprocessorError::Custom(s) => s,
            HtmlPreprocessorError::TooManyTagArguments(p) => format!("Too many tag arguments at {}:{}.", p.line, p.column),
            HtmlPreprocessorError::IllegalAttributeValue(s, p) => format!("Illegal attribute value {} found at {}:{}.", s, p.line, p.column),
        }),
    };
    Ok(tex)
}
