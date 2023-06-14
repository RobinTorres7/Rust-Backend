#[macro_use]
extern crate diesel;
//this way, i bring schema data//
pub mod models;
pub mod shema;

// Importamos lo necesario para levantar un servidor web y exponer endpoints
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use std::env;

use tera::Tera;

use diesel::pg::PgConnection;
use diesel::prelude::*;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use self::models::{NewPost, Post};
use self::schema::posts::dsl::*;

//

#[get("/tera_test")]

async fn tera_test(template_manager: web::Data<tera::Tera>) -> Responder {
    //Creamos  un contexto para pasarle datos al template, el contexto es mutable porque podria cambiar
    let mut ctx = tera::Context::new();

    // Enviamos el template que queremos localizando por su nombre
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("tera_test.html", &ctx).unwrap())
}

// Endpoint GET que devuelve texto, utiliamos el Macro GET para indicar el verbo HTTP
#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    //Traemos el pool para disponer de la conexion a la BBDD
    let conn = pool.get().expect("Problems to bring pool conection");
    //Consulta para obtener todos los registros
    //El match responde en caso de exito o error en la consulta
    match web::block(move || posts.load::<Posts>(&conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            // Enviamos, a traves del contexto, los datos al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("post", &data);

            //Pasamos los datos al template index.html
            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("index.html", &ctx).unwrap())
        }
        Error(error) => HttpResponse::Ok().body("Error receiving data."),
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Problems to bring pool conection");

    let new_post = NewPost {
        title: "mi post",
        slug: "mi-post",
        body: "Hola mundo, este es mi primer post",
    };

    match web::block(move || Post::create_post(&conn, &item)).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Error(error) => HttpResponse::Ok().body("Error receiving data."),
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>,
) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");
    let url_slug = blog_slug.into_inner();

    match wen::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&conn)).await {
        ok(data) => {
            let data = data.unwrap();

            //Si el post no existe devolvemos 404
            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }
            let data = &data[0];

            //enviamos, atraves del contexto, los datos del post al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("post", data);

            HttpResponse::OK()
                .content_type("text/html")
                .body(template_manager.render("post.html", &ctx).unwrap())
        }
        Err(err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

// Macro para establecer la aplicación del tipo Web, la misma debe ser asíncrona
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Db_url doesn't find");
    let port = env::var("PORT").expect("Db_url doesn't find");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    // El POOL sirve para compartir la conexión con otros servicios
    let pool = Pool::builder()
        .build(connection)
        .expect("No se pudo construir el Pool");

    // Exponemos los endpoints que indiquemos
    HttpServer::new(move || {
        //Instanciamos TERA y le indicamos en que directorio buscar los templates
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .service(index)
            .service(new_post)
            .service(get_post)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
    })
    // Indicamos el host y el puerto donde escuchará el servidor
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await

    // let conn = PgConnection::establish(&db_url).expect("We didn't connect to data base");

    // use self::models::{NewPost, Post, PostSimplify};
    // use self::schema::posts;
    // use self::schema::posts::dsl::*;

    // //the next line of code is how i would eliminate a post in my data base//

    // diesel::delete(posts.filter(slug.eq("tercer-post")))
    //     .execute(&conn)
    //     .expect("failed the elimination");

    // //the next line of code is how i would update data base//

    // let update = diesel::update(posts.filter(id.eq(3)))
    //     .set((
    //         body.eq("This post has been edited"),
    //         title.eq("My thrid blog-post"),
    //     ))
    //     .get_result::<Post>(&conn)
    //     .expect("Error in update");

    // // let new_post = NewPost {
    // //     title: "Mi primer post",
    // //     body: "Vamos con toda",
    // //     slug: "primer-post",
    // // };

    // // let post: Post = diesel::insert_into(post::table)
    // //     .values(&new_post)
    // //     .get_result(&conn)
    // //     .expect("Failed");

    // //Select * from posts
    // println!("Query whitout limits");
    // let posts_result = posts
    //     .load::<Post>(&conn)
    //     .expect("Error to ejecute the query");

    // for post in posts_result {
    //     println!("{:?}", post);
    // }

    // //     println!("Query  limits");
    // //     let posts_result = posts
    // //         .order(id)
    // //         .limit(1)
    // //         .load::<Post>(&conn)
    // //         .expect("Error to ejecute the query");

    // //     for post in posts_result {
    // //         println!("{:?}", post);
    // //     }

    // //     println!("Query  wiht especific columns");
    // //     let posts_result = posts
    // //         .select((title, body))
    // //         .limit(1)
    // //         .load::<PostSimplify>(&conn)
    // //         .expect("Error to ejecute the query");

    // //     for post in posts_result {
    // //         println!("{:?}", post.title);
    // //     }
}
