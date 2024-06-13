pub async fn http_main(settings: &crate::settings::Settings) -> std::io::Result<()> {
    // settings up the telemetry
    // read the config with some db pool settings
    // create the http server with axum
    //
    println!("settings: {:?}", settings);

    let ctx = match service::ctx::Ctx::new(settings).await {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("unable to connect to postgresql: {:?}", err);
            std::process::exit(1);
        }
    };

    let listener = tokio::net::TcpListener::bind(std::net::SocketAddr::new(
        settings
            .service
            .bind
            .parse()
            .expect("unexpected value for `bind` address"),
        settings.service.port,
    ))
    .await?;

    eprintln!(
        "##### Server Started ### --> {}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );

    axum::serve(listener, crate::router::routes(ctx).await).await?;
    Ok(())
}
