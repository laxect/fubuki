use async_std::task;

#[async_std::main]
pub async fn main() -> Result<(), ()> {
    let task_pub = task::spawn(async {
        let pub_result = index_gen::update_pubsubhubbub().await;
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
    let task_files = task::spawn(async move {
        index_gen::read_files().expect("error!");
    });
    task_pub.await;
    task_files.await;
    Ok(())
}
