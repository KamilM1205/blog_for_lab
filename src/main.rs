use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::{config::PersistentSession, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware, web, App, HttpServer,
};
use log::info;
use methods_2lab::db::init_db;
use methods_2lab::errors::DbError;
use methods_2lab::{article, auth_handler, author, blog, comment, register_handler};
use methods_2lab::{config::ServerConfig, utils};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ServerConfig::load().unwrap();
    methods_2lab::log::init_logger(&config).unwrap();

    info!(
        "Initializing server at {}:{}",
        config.cd.host, config.cd.port
    );

    init_db(&config.pg.get().await.map_err(DbError::PoolError).unwrap())
        .await
        .ok();

    let config_move = config.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config_move.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(utils::SECRET_KEY.as_bytes()),
                )
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                .cookie_name("simple-blog".to_string())
                .cookie_secure(false)
                .cookie_domain(Some(config_move.cd.host.clone()))
                .cookie_path("/".to_string())
                .build(),
            )
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/register").route(web::post().to(register_handler::register_author)),
            )
            .service(
                web::resource("/auth")
                    .route(web::post().to(auth_handler::login))
                    .route(web::delete().to(auth_handler::logout))
                    .route(web::get().to(auth_handler::get_me)),
            )
            .service(
                web::resource("/blogs")
                    .route(web::post().to(blog::add_blog))
                    .route(web::get().to(blog::get_blogs)),
            )
            .service(
                web::scope("/article")
                    .service(web::resource("").route(web::get().to(article::get_article)))
                    .service(
                        web::resource("/{article_id}")
                            .route(web::post().to(article::add_article))
                            .route(web::delete().to(article::delete_article)),
                    ),
            )
            .service(
                web::scope("/author").service(
                    web::resource("/{author_id}").route(web::get().to(author::get_author)),
                ),
            )
            .service(
                web::scope("/comment")
                    .service(web::resource("").route(web::post().to(comment::add_comment)))
                    .service(
                        web::resource("/{comment_id}")
                            .route(web::delete().to(comment::delete_comment)),
                    )
                    .service(
                        web::resource("/{article_id}").route(web::get().to(comment::get_comments)),
                    ),
            )
    })
    .bind((config.cd.host, config.cd.port))?
    .run()
    .await
}
