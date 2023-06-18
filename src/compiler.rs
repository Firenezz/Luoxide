use pest::Parser;
use pest_derive::Parser;

pub mod lexer;

#[derive(Parser)]
#[grammar = "grammar/luoxidant.pest"]
pub struct LuoxidantParser;

pub fn parse(source: &str) -> Result<(), pest::error::Error<Rule>> {
    //let pairs = LuoxidantParser::parse(Rule::program, source)?;
    /*for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        println!("Inner:   {:?}", pair.into_inner());
    }*/
    Ok(())
}