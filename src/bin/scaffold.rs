use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: scaffold <command> <name>");
        println!("Commands:");
        println!("  feature <name>     Generate feature boilerplate (mod, handler, service, repo)");
        println!("  middleware <name>  Generate middleware boilerplate");
        println!("  extractor <name>   Generate extractor boilerplate");
        return;
    }

    let command = &args[1];
    let name = &args[2];

    match command.as_str() {
        "feature" => generate_feature(name),
        "middleware" => generate_middleware(name),
        "extractor" => generate_extractor(name),
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

fn generate_feature(name: &str) {
    let base_path = format!("src/features/{}", name);
    if Path::new(&base_path).exists() {
        println!("Feature {} already exists.", name);
        return;
    }

    fs::create_dir_all(&base_path).expect("Failed to create feature directory");

    // mod.rs
    let mod_content = format!(
        r#"pub mod handler;
pub mod repository;
pub mod service;

pub use handler::router;
"#
    );
    fs::write(format!("{}/mod.rs", base_path), mod_content).expect("Failed to write mod.rs");

    // handler.rs
    let struct_name = to_camel_case(name);
    let handler_content = format!(
        r#"use axum::{{
    extract::{{State, Path}},
    http::StatusCode,
    routing::{{get, post}},
    Json, Router,
}};
use sea_orm::DatabaseConnection;
use serde::{{Deserialize, Serialize}};
use validator::Validate;

use crate::{{
    extractors::ValidatedJson,
    infra::routes::AppState,
    types::error::AppError,
}};
use super::service::{struct_name}Service;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct Create{struct_name}Request {{
    // Add fields here
}}

#[derive(Debug, Serialize)]
pub struct {struct_name}Response {{
    // Add fields here
}}

pub fn router() -> Router<AppState> {{
    Router::new()
        // .route("/", get(list).post(create))
        // .route("/{{id}}", get(get_by_id))
}}

// Add handler functions here
"#
    );
    fs::write(format!("{}/handler.rs", base_path), handler_content).expect("Failed to write handler.rs");

    // service.rs
    let service_content = format!(
        r#"use sea_orm::DatabaseConnection;
use crate::types::error::AppError;
use super::repository::{struct_name}Repository;

pub struct {struct_name}Service;

impl {struct_name}Service {{
    // Add business logic methods here
}}
"#
    );
    fs::write(format!("{}/service.rs", base_path), service_content).expect("Failed to write service.rs");

    // repository.rs
    let repo_content = format!(
        r#"use sea_orm::DatabaseConnection;

pub struct {struct_name}Repository;

impl {struct_name}Repository {{
    // Add database access methods here
}}
"#
    );
    fs::write(format!("{}/repository.rs", base_path), repo_content).expect("Failed to write repository.rs");

    // update src/features/mod.rs
    let mut features_mod = OpenOptions::new()
        .append(true)
        .open("src/features/mod.rs")
        .expect("Failed to open src/features/mod.rs");
    writeln!(features_mod, "pub mod {};", name).expect("Failed to append to src/features/mod.rs");

    // update src/infra/routes.rs
    let routes_path = "src/infra/routes.rs";
    if let Ok(content) = fs::read_to_string(routes_path) {
        let kebab_name = to_kebab_case(name);
        let target = r#".nest("/health", crate::features::health::router())"#;
        if content.contains(target) {
            let replacement = format!(
                "{}\n        .nest(\"/{}\", crate::features::{}::router())",
                target, kebab_name, name
            );
            let updated_content = content.replace(target, &replacement);
            fs::write(routes_path, updated_content).expect("Failed to write src/infra/routes.rs");
            println!("Feature route nested in src/infra/routes.rs successfully.");
        } else {
            println!("Warning: Could not find target route to nest feature in src/infra/routes.rs");
        }
    }

    println!("Feature {} generated successfully.", name);
}

fn generate_middleware(name: &str) {
    let path = format!("src/middleware/{}.rs", name);
    if Path::new(&path).exists() {
        println!("Middleware {} already exists.", name);
        return;
    }

    let content = format!(
        r#"use axum::{{
    extract::Request,
    middleware::Next,
    response::Response,
}};

pub async fn {name}_middleware(
    request: Request,
    next: Next,
) -> Response {{
    // Middleware logic here
    let response = next.run(request).await;
    response
}}
"#
    );

    fs::write(&path, content).expect("Failed to write middleware file");

    // update src/middleware/mod.rs
    let mut middleware_mod = OpenOptions::new()
        .append(true)
        .open("src/middleware/mod.rs")
        .expect("Failed to open src/middleware/mod.rs");
    writeln!(middleware_mod, "pub mod {};", name).expect("Failed to append to src/middleware/mod.rs");

    println!("Middleware {} generated successfully.", name);
}

fn generate_extractor(name: &str) {
    let path = format!("src/extractors/{}.rs", name);
    if Path::new(&path).exists() {
        println!("Extractor {} already exists.", name);
        return;
    }

    let struct_name = to_camel_case(name);
    let content = format!(
        r#"use axum::{{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
}};
use crate::types::error::AppError;

pub struct {struct_name};

#[async_trait]
impl<S> FromRequestParts<S> for {struct_name}
where
    S: Send + Sync,
{{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {{
        // Extractor logic here
        Ok({struct_name})
    }}
}}
"#
    );

    fs::write(&path, content).expect("Failed to write extractor file");

    // update src/extractors/mod.rs
    let mut extractors_mod = OpenOptions::new()
        .append(true)
        .open("src/extractors/mod.rs")
        .expect("Failed to open src/extractors/mod.rs");
    writeln!(extractors_mod, "pub mod {};", name).expect("Failed to append to src/extractors/mod.rs");

    println!("Extractor {} generated successfully.", name);
}

fn to_camel_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

fn to_kebab_case(s: &str) -> String {
    s.replace('_', "-")
}
