#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let task_files = tokio::spawn(async move {
        posts_gen::read_files().expect("error!");
    });
    let task_pub = tokio::spawn(async {
        let pub_result = posts_gen::update_pubsubhubbub().await;
        println!("pub task finish.");
        match pub_result {
            Ok(resp) => {
                println!("update success\n{:?}", resp);
            }
            Err(e) => {
                println!("update failed\n{:?}", e);
            }
        };
    });
    task_files.await.expect("task files failed");
    task_pub.await.expect("task pub failed");
    Ok(())
}
