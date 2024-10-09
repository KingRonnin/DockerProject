#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::handlers::student_handler::{create_student, get_students};

    #[actix_web::test]
    async fn test_get_student() {
        let app = test::init_service(App::new().route("/student", web::get().to(get_students))).await;
        let req = test:TestRequest::get().uri("/student").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}