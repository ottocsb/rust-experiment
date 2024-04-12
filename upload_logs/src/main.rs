use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use reqwest;
use reqwest::blocking::Client;

fn upload_logs(log_dir: &Path) {
    let files = fs::read_dir(log_dir).expect("无法读取日志目录");
    println!("\n-----------------");
    println!("当前时间：{:?}", chrono::Local::now());
    println!("开始上传日志");

    for file in files {
        let file = file.expect("找不到文件");
        let file_path = file.path();

        let content = fs::read_to_string(&file_path).expect("无法读取文件");
        let client = Client::new();
        let res = client.post("http://localhost:3000/upload")
            .body(content)
            .send();
        match res {
            Ok(resp) => println!("上传{}成功: {}", file_path.display(), resp.status()),
            Err(err) => println!("上传{}失败: {}", file_path.display(), err),
        }
    }
}

fn main() {
    let log_dir = Path::new("C:\\logs");
    loop {
        upload_logs(&log_dir);
        // thread::sleep(Duration::from_secs(24 * 60 * 60)); // 睡眠24小时
        thread::sleep(Duration::from_secs(10)); // 睡眠10秒
    }
}