use crate::templates::{hello::HelloTemplate, todo::TodoTemplate};

pub async fn root_handler() -> HelloTemplate<'static> {
    HelloTemplate { name: "wanker" }
}


pub async fn todomain() -> TodoTemplate<'static> {
    TodoTemplate { name: "master" }
}
