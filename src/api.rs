

#[derive(Deserialize, Debug)]
pub enum Goal {
    MINIMIZE,
    MAXIMIZE,
}

#[derive(Deserialize)]
pub struct Param {
    #[serde(rename = "parameterName")]
    pub parameter_name: String,
    pub r#type: ParameterType
}

#[derive(Deserialize)]
pub enum ParameterType {
    DOUBLE,
    INTEGER,
    CATEGORICAL,
    DISCRETE
}

#[derive(Deserialize)]
pub struct Study {
    pub goal: Goal,
    pub params: Vec<Param>
}
