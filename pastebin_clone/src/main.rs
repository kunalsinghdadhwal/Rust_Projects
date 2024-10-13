use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::Mutex;
use tokio::task::spawn_blocking;
struct AppState {
    db: Mutex<Connection>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("index.html"))
}

async fn submit(content: web::Form<FormData>, data: web::Data<AppState>) -> impl Responder {
    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let wow = token.clone();
    let result = spawn_blocking(move || {
        let conn = data.db.lock().unwrap();
        conn.execute(
            "INSERT INTO pastes (token, content) VALUES (?, ?)",
            params![&token, &content.content],
        )
    })
    .await;

    match result {
        Ok(Ok(_)) => HttpResponse::SeeOther()
            .header("Location", format!("/paste/{}", wow))
            .finish(),
        Ok(Err(e)) => {
            eprintln!("Failed to insert into database: {}", e);
            HttpResponse::InternalServerError().finish()
        }
        Err(_) => {
            eprintln!("Task failed to execute");
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_paste(token: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let content = conn
        .query_row(
            "SELECT content FROM pastes WHERE token = ?",
            params![token.to_string()],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "Paste Not Found".to_string());

    HttpResponse::Ok().body(format!("<pre>{}</pre>", content))
}

async fn delete_paste(token: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    conn.query_row(
        "DELETE FROM pastes WHERE token = ?",
        params![token.to_string()],
        |row| row.get::<_, String>(0),
    )
    .unwrap_or_else(|_| "Paste Not found".to_string());

    HttpResponse::Ok().body("<pre>Deleted Successfully</pre>")
}

async fn style_css() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = PathBuf::from("src/style.css");
    Ok(NamedFile::open(path)?)
}
#[derive(serde::Deserialize)]
struct FormData {
    content: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Connection::open("pastes.db").expect("Failed to open database");
    db.execute(
        "CREATE TABLE IF NOT EXISTS pastes (token TEXT PRIMARY KEY, content TEXT)",
        params![],
    )
    .expect("Failed to create table");
    let app_state = web::Data::new(AppState { db: Mutex::new(db) });
    println!("Server running on http://127.0.0.1:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("./style.css").to(style_css))
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
            .service(
                web::resource("/paste/{token}")
                    .route(web::get().to(get_paste))
                    .route(web::delete().to(delete_paste)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}
