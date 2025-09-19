use crate::tool::scan::scanning;



mod tool;

#[tokio::main]
async fn main() {
    scanning().await;
}
