use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct State {
    pub todos: Vec<Todo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub description: String,
}
