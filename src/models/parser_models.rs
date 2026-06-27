
#[derive(PartialEq, Debug)]
pub struct ParsedData<'a>{
    pub key: &'a str,
    pub value: Vec<ValuePart<'a>>,
}

#[derive(PartialEq, Debug)]
pub enum ValuePart<'a>{
    Translatable(&'a str),
    SysData(&'a str),
    NewLine,
}

impl<'a> ParsedData<'a>{
    pub fn new(key: &'a str, value: Vec<ValuePart <'a> >) -> Self{
        Self { key, value }
    }
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_new(){
        let a = "key";
        let b = "value";
        let c = ValuePart::Translatable(b);
        let d = ParsedData::new(a, vec![c, ValuePart::NewLine]);

        assert_eq!(d.key, a);
        assert_eq!(d.value, vec![ValuePart::Translatable(b), ValuePart::NewLine])
    }
}

