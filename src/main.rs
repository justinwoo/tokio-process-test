fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let child = tokio_process::Command::new("ls")
            .spawn()
            .expect("Error running get-icon");

        child.await.expect("Error waiting on get-icon");
    });
}
