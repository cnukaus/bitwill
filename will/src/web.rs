use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use crate::spending::{UserModel, ProjectSpend, GrowthType};

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
    // In a real application, you'd fetch the user from a database
    let mut user = match UserModel::new(user_id.into(), 5) {
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

pub async fn run_server() -> std::io::Result<()> {
    println!("Starting web server at http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/users", web::post().to(create_user))
            .route("/users/{user_id}/projects", web::post().to(add_project))
            .route("/users/{user_id}/projection", web::get().to(calculate_projection))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 