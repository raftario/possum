use possum::{
    ast::{self, Tokens},
    lexer::{self, Span, Token},
};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

fn js_str_debug<T: Debug>(v: T) -> JsValue {
    JsValue::from_str(&format!("{:#?}", v))
}

#[wasm_bindgen]
pub fn compile(source: &str) -> Result<Vec<u8>, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let tokens = lexer::lex(source)
        .collect::<Result<Vec<Token>, (lexer::Error, Span)>>()
        .map_err(js_str_debug)?;
    let tokens = Tokens::new(&tokens);
    let ast = ast::parse(&tokens).map_err(js_str_debug)?;

    Ok(format!("{:#?}", ast).into_bytes())
}
