macro_rules! route {
    (get $path:expr => $handler:path) => {
        move |cfg: &mut web::ServiceConfig| {
            cfg.service(web::resource($path).route(web::get().to($handler)));
        }
    };
    (post $path:expr => $handler:path) => {
        move |cfg: &mut web::ServiceConfig| {
            cfg.service(web::resource($path).route(web::post().to($handler)));
        }
    };
}


macro_rules! routes_map {
    ($($r:expr);* $(;)?) => {{
        move |cfg: &mut web::ServiceConfig| {
            $(
                cfg.configure($r);
            )*
        }
    }};
}

macro_rules! start_server {
    ($server_config:expr, $port:expr) => {
        HttpServer::new(|| {
            App::new().configure($server_config)})
            .bind($port)?
            .run()
            .await
    }
}

macro_rules! grouped_routes {
    ($description:expr => { $($content:expr);* $(;)? }) => {{
        move |cfg: &mut web::ServiceConfig| {
            $(
                cfg.configure($content);
            )*
        }
    }};
}

macro_rules! scope_routes {
    ($scope:expr, { $($r:expr);* $(;)? }) => {{
        let scoped = web::scope($scope).configure(
            move |cfg| {
                $(
                    cfg.configure($r);
                )*
            }
        );
        move |cfg: &mut web::ServiceConfig| {
            cfg.service(scoped);
        }
    }};
}

