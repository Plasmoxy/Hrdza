use web_view::*;

fn js_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn main() {
    let html = format!(
        include_str!("dist/index.html"),
        script = include_str!("dist/src.e31bb0bc.js")
    );
	
    web_view::builder()
        .title("My Project")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
        
        
}