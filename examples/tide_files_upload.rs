const UPLOAD_PAGE_HTML: &str = r#"<html>
    <head><title>Upload Files</title></head>
    <body style="display: flex; align-items: center; justify-content: center;">
        <form target="/" method="post" enctype="multipart/form-data">
            <input type="file" multiple name="file"/>
            <button type="submit">Submit</button>
        </form>
    </body>
</html>"#;

// FIXME post file handler not working
fn main() -> tide::Result<()> {
    tide::log::start();
    tide::log::info!("server_ip: {}", get_server_ip().unwrap());

    futures::executor::block_on(async {
        let mut app = tide::new();
        app.at("/").get(handle_get_index).post(handle_post_files);
        app.listen("0.0.0.0:8666").await?;
        Ok(())
    })
}

fn get_server_ip() -> Result<String, Box<dyn std::error::Error>>{
    let ifconfig = std::process::Command::new("ifconfig")
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let pipe_grep_res = std::process::Command::new("grep")
        .arg("inet 192.168")
        .stdin(ifconfig.stdout.unwrap())
        .output()?;
    let grep_str = String::from_utf8(pipe_grep_res.stdout)?;
    let re = regex::Regex::new(r"192.168.\d.\d")?;
    let ip = re.find(grep_str.as_str()).unwrap().as_str();
    Ok(ip.to_string())
}

async fn handle_get_index(_: tide::Request<()>) -> tide::Result {
    Ok(tide::Response::builder(200)
        .content_type(tide::http::mime::HTML)
        .body(UPLOAD_PAGE_HTML)
        .build())
}

// FIXME not working
async fn handle_post_files(mut req: tide::Request<()>) -> tide::Result {
    dbg!(&req);
    let path: String = req.param("file")?;
    let mut file = async_std::fs::OpenOptions::new().create(true).write(true).open(path).await?;
    async_std::io::copy(&mut req, &mut file).await?;
    Ok(tide::Redirect::temporary("/").into())
}
