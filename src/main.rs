use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::{config::PersistentSession, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware, web, App, HttpServer,
};
use log::info;
use methods_2lab::blog;
use methods_2lab::db::init_db;
use methods_2lab::errors::DbError;
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
                .cookie_name("simple-store".to_string())
                .cookie_secure(false)
                .cookie_domain(Some(config_move.cd.host.clone()))
                .cookie_path("/".to_string())
                .build(),
            )
            .wrap(middleware::Logger::default())
            .service(
                // web::scope("/api")
                //     .service(
                //         web::resource("/register")
                //             .route(web::post().to(register_handler::register_user)),
                //     )
                //     .service(
                //         web::resource("/auth")
                //             .route(web::post().to(auth_handler::login))
                //             .route(web::delete().to(auth_handler::logout))
                //             .route(web::get().to(auth_handler::get_me)),
                //     ),
                web::resource("/blog")
                    .route(web::post().to(blog::add_blog))
                    .route(web::get().to(blog::get_blogs)),
            )
    })
    .bind((config.cd.host, config.cd.port))?
    .run()
    .await
}
