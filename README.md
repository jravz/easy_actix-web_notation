# easy_actix-web_notation
Write easily readable routes for actix-web within Rust.

Imagine presenting starting your server and presenting your routes in an easy readable fashion.
Be able to scope and group your routes.

      //start server macro takes in the route details and the server / port details
      start_server!(
            //all routes defined within the routes_map macro
            routes_map!(
      
                //you wnat to group a set of routes within a description like all UTILS within a block. Do that using grouped_routes!
               grouped_routes!(
               "UTILS" => {
                  //route! is the most basic macro unit that can be defined at any level within routes_map or within grouped_routes and scope_routes.
                  route!(get "/robots.txt" => robots_txt);
                  route!(get "/api/info_details" => get_info_details);
               });
      
               //manage scoping a set of routes and grouped_routes using scope_routes
               scope_routes!( "/api", {
                  route!(post "/endpoint01/{dynamic_segment}" => endpoint01);
                  grouped_routes!(
                  "MOD MIFOR" => {
                     route!(post "/endpoint02" => endpoint02);
                     route!(post "/endpoint03" => endpoint03);
                  } );
      
                ), 
          //the port details
         "0.0.0.0:80")
