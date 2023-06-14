use super::schema::posts;
use diesel::prelude::*;

//permitimos a las estructuras manipular datos en formatos JSON
use serde::{Deserialize, Serialize};

#[derive(Queryble, Debug, Deserialize, Serialize)]
//Queryble mean i would convert inside to a row SQL
pub struct PostSimplify {
    //pub id: i32,
    pub title: String,
    // pub slug: String,
    pub body: String,
}
#[derive(Queryble, Debug, Deserialize, Serialize)]

pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

//life time usar los str debtro de las estructuras//

#[derive(Insertable)]
#[table_name = "posts"]

pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}
#[derive(Clone, Serialize, Deserialize, Debug)]

pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

impl Post {
    //funcion para crear un String con el formato de un 'slug', minuscula y separado por guion medio//
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    //funcion para crear un  nuevo Post en la BBDD a partir de datos de entrada//
    pub fn create_post<'a>(
        conn: &PgConnection,
        post: NewPostHandler,
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title.clone());

        let new_post = NewPost {
            title: &post.title,
            slug: &slug,
            body: &post.body,
        };
        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(conn)
    }
}
