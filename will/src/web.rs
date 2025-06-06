use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use crate::spending::{UserModel, ProjectSpend, GrowthType};
use std::env;
use actix_cors::Cors;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    project_name: String,
    daily_spend: f64,
    growth_rate: f64,
    growth_type: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    user_id: String,
    projection_years: u32,
}

#[derive(Serialize)]
pub struct ProjectionResponse {
    yearly_totals: Vec<YearlyTotal>,
}

#[derive(Serialize)]
pub struct YearlyTotal {
    year: u32,
    total: f64,
}

async fn create_user(req: web::Json<CreateUserRequest>) -> impl Responder {
    match UserModel::new(req.user_id.clone(), req.projection_years) {
        Ok(user) => {
            // In a real application, you'd store this in a database
            HttpResponse::Ok().json(user)
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn add_project(
    user_id: web::Path<String>,
    req: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let growth_type = match req.growth_type.as_str() {
        "compound" => GrowthType::Compound,
        "flat" => GrowthType::Flat,
        custom => GrowthType::Custom(custom.to_string()),
    };

    match ProjectSpend::new(
        req.project_name.clone(),
        req.daily_spend,
        req.growth_rate,
        growth_type,
    ) {
        Ok(project) => {
            // In a real application, you'd add this to the user's projects in a database
            HttpResponse::Ok().json(project)
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn calculate_projection(user_id: web::Path<String>) -> impl Responder {
    // Extract the String from Path
    let user_id = user_id.into_inner();
    
    // In a real application, you'd fetch the user from a database
    let mut user = match UserModel::new(user_id, 5) {
        Ok(u) => u,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    // Example projects - in a real app, these would come from a database
    let project1 = ProjectSpend::new(
        "Project A".into(),
        100.0,
        0.05,
        GrowthType::Compound,
    ).unwrap();
    
    let project2 = ProjectSpend::new(
        "Project B".into(),
        50.0,
        0.03,
        GrowthType::Flat,
    ).unwrap();

    user.add_project(project1);
    user.add_project(project2);

    match user.calculate_total_spend() {
        Ok(totals) => {
            let yearly_totals: Vec<YearlyTotal> = totals
                .into_iter()
                .map(|(year, total)| YearlyTotal { year, total })
                .collect();
            
            HttpResponse::Ok().json(ProjectionResponse { yearly_totals })
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Projection API! Use /users to create a new user.")
}

pub async fn run_server() -> std::io::Result<()> {
    // Get port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let host = env::var("HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());
    
    println!("Starting web server at http://{}:{}", host, port);
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/users", web::post().to(create_user))
            .route("/users/{user_id}/projects", web::post().to(add_project))
            .route("/users/{user_id}/projection", web::get().to(calculate_projection))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
} 