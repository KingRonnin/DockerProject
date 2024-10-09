use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::connection::establish_connection;
use crate::models:student::{Student, NewStudent};

pub async fn create_student(student: web::Json<NewStudent>) -> impl Responder {
    use crate::schema::students;

    let conn = establish_connection();
    diesel::insert_into(students::table)
        .values(&*student)
        .execute(&conn)
        .expect("Failed to insert student record");

    HttpResponse::Ok().json("Student created")
}

pub async fn get_students() -> impl Responder {
    use crate::schema::students::dsl::*;

    let conn = establish_connection();
    let results = students
        .load::<Student>(&conn)
        .expect("Failed to load record(s)")

        HttpResponse::Ok().json(results)
}

