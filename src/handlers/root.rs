use crate::templates::hello::HelloTemplate;

pub async fn root_handler() -> HelloTemplate<'static> {
    HelloTemplate { name: "wanker" }
}


