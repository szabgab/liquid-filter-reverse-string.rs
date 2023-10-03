use liquid_core::{Result, Value, ValueView, Filter, Runtime, Display_filter, ParseFilter, FilterReflection};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "reversestr",
    description = "Reverse string.",
    parsed(ReverseStrFilter)
)]
pub struct ReverseStr;


#[derive(Debug, Default, Display_filter)]
#[name = "reversestr"]
pub struct ReverseStrFilter;

impl Filter for ReverseStrFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let text = input.to_kstr();
        let reversed: String = text.chars().rev().collect();
        Ok(Value::scalar(reversed.to_string()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_reverse() {
        let text = "{{ text | reversestr}}";
        let globals = liquid::object!({
            "text": "Hello!",
        });
        let template = liquid::ParserBuilder::with_stdlib()
            .filter(ReverseStr)
            .build()
            .unwrap()
            .parse(text)
            .unwrap();
        let output = template.render(&globals).unwrap();
        assert_eq!(output, "!olleH".to_string());
    }
}

