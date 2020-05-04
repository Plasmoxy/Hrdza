use web_view::*;

fn main() {
    let html = include_str!("index.html");
	
    web_view::builder()
        .title("My Project")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
        
    
}