use std::fmt;
use serde_json::Value;
#[derive(Debug)]
pub struct MoexEgine{
    pub id: i64,
    pub name: String,
    pub title: String
}

impl fmt::Display for MoexEgine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{}, name:{}, title:{}", &self.id, &self.name, &self.title);
        Ok(())
    }
}

pub fn from_json_val(val:&Value) -> MoexEgine {
    MoexEgine{
        id: val[0].as_i64().unwrap(),
        name: val[1].as_str().unwrap().to_string(),
        title: val[2].as_str().unwrap().to_string()
    }
}
