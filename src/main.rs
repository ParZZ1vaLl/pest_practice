use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let successful_parse = Grammar::parse(Rule::field, "-273.15")?;
    println!("{:?}", successful_parse);

    let successful_parse_record = Grammar::parse(Rule::record, "-273.15,98.6,0.0")?;
    println!("Record parse: {:?}", successful_parse_record);

    let successful_parse_file = Grammar::parse(Rule::file, "-273.15,98.6,0.0\n42.0,-50.5,123.4\n")?;
    println!("File parse: {:?}", successful_parse_file);

    let unsuccessful_parse = Grammar::parse(Rule::field, "this is not a number");
    println!("{:?}", unsuccessful_parse);

    Ok(())
}
