use actix_web::{ HttpResponse, web::{ get, ServiceConfig } };

const AMIYA: &'static str = r#"<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="display: flex; justify-content: center; align-items:center; width: 100%; min-height: 100vh; margin: 0; padding: 20px; box-sizing: border-box;">
    <img 
        src="https://meme.saltyaom.com/meme/amiya%20-%20twitter%20blushing.jpg" 
        alt="Amiya"
        style="max-width: 660px; width: 100%; object-fit: contain;"
    />
</body>
</html>
"#;

pub async fn fallback() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(AMIYA)
}

pub fn status(config: &mut ServiceConfig) {
    config
        .route("/", get().to(fallback));
}
