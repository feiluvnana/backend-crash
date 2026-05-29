use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    // Support commands:
    // make g:feature name=xxx -> g feature xxx
    // make g:resource name=xxx -> g resource xxx
    // default (e.g. g xxx) -> g resource xxx
    let (command, feature_name) = if args.len() == 2 {
        ("resource", &args[1])
    } else {
        (args[1].as_str(), &args[2])
    };

    if !is_valid_snake_case(feature_name) {
        eprintln!("Error: Feature name must be in snake_case format (e.g., 'user_profile').");
        std::process::exit(1);
    }

    match command {
        "feature" => generate_feature(feature_name),
        "resource" => generate_resource(feature_name),
        _ => {
            eprintln!("Error: Unknown command '{}'.", command);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  make g:feature name=<name>   (Creates a basic feature starter)");
    eprintln!("  make g:resource name=<name>  (Creates a NestJS-like CRUD resource)");
}

fn generate_feature(feature_name: &str) {
    let camel_case = to_camel_case(feature_name);
    let kebab_case = to_kebab_case(feature_name);

    let target_dir_str = format!("src/features/{}", feature_name);
    let target_dir = Path::new(&target_dir_str);

    if target_dir.exists() {
        eprintln!("Error: Directory '{}' already exists.", target_dir.display());
        std::process::exit(1);
    }

    println!("Creating feature directory: {}...", target_dir.display());
    if let Err(e) = fs::create_dir_all(target_dir) {
        eprintln!("Failed to create directory: {}", e);
        std::process::exit(1);
    }

    // Write mod.rs
    let mod_content = "pub mod dto;\npub mod handler;\npub mod service;\n";
    let _ = write_file(&target_dir.join("mod.rs"), mod_content);

    // Write dto.rs
    let dto_content = format!(
        r#"use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct Create{Request}Request {{
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
}}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct {Response}Response {{
    pub id: i32,
    pub name: String,
}}
"#,
        Request = camel_case,
        Response = camel_case
    );
    let _ = write_file(&target_dir.join("dto.rs"), &dto_content);

    // Write handler.rs
    let handler_content = format!(
        r#"use axum::{{Json, extract::State, extract::Query, http::StatusCode}};
use sea_orm::DatabaseConnection;

use crate::{{
    infra::{{
        error::{{AppError, ErrorResponse}},
        extractor::ValidatedJson,
        pagination::{{PaginationParams, PaginatedResponse}},
    }},
    features::{FeatureName}::{{
        dto::{{Create{CamelName}Request, {CamelName}Response}},
        service::{CamelName}Service,
    }},
}};

#[utoipa::path(
    post,
    path = "/api/{KebabName}",
    request_body = Create{CamelName}Request,
    responses(
        (status = 201, description = "Created successfully", body = {CamelName}Response),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn create(
    State(db): State<DatabaseConnection>,
    ValidatedJson(payload): ValidatedJson<Create{CamelName}Request>,
) -> Result<(StatusCode, Json<{CamelName}Response>), AppError> {{
    let result = {CamelName}Service::create_item(&db, &payload.name).await?;

    Ok((
        StatusCode::CREATED,
        Json({CamelName}Response {{
            id: result,
            name: payload.name,
        }}),
    ))
}}
"#,
        FeatureName = feature_name,
        CamelName = camel_case,
        KebabName = kebab_case
    );
    let _ = write_file(&target_dir.join("handler.rs"), &handler_content);

    // Write service.rs
    let service_content = format!(
        r#"use sea_orm::DatabaseConnection;
use crate::infra::error::AppError;

pub struct {CamelName}Service;

impl {CamelName}Service {{
    /// Example service method
    pub async fn create_item(db: &DatabaseConnection, _name: &str) -> Result<i32, AppError> {{
        // Implement database logic here
        Ok(1)
    }}
}}
"#,
        CamelName = camel_case
    );
    let _ = write_file(&target_dir.join("service.rs"), &service_content);

    register_feature_in_mod(feature_name);

    println!("Feature '{}' generated successfully!", feature_name);
}

fn generate_resource(feature_name: &str) {
    let camel_case = to_camel_case(feature_name);
    let kebab_case = to_kebab_case(feature_name);

    let target_dir_str = format!("src/features/{}", feature_name);
    let target_dir = Path::new(&target_dir_str);

    if target_dir.exists() {
        eprintln!("Error: Directory '{}' already exists.", target_dir.display());
        std::process::exit(1);
    }

    println!("Creating resource directory: {}...", target_dir.display());
    if let Err(e) = fs::create_dir_all(target_dir) {
        eprintln!("Failed to create directory: {}", e);
        std::process::exit(1);
    }

    // Write mod.rs
    let mod_content = "pub mod dto;\npub mod handler;\npub mod service;\n";
    let _ = write_file(&target_dir.join("mod.rs"), mod_content);

    // Write dto.rs
    let dto_content = format!(
        r#"use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct Create{CamelName}Request {{
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
}}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct Update{CamelName}Request {{
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
}}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct {CamelName}Response {{
    pub id: i32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}}

impl From<crate::db::models::{SnakeName}::Model> for {CamelName}Response {{
    fn from(model: crate::db::models::{SnakeName}::Model) -> Self {{
        Self {{
            id: model.id,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }}
    }}
}}
"#,
        CamelName = camel_case,
        SnakeName = feature_name
    );
    let _ = write_file(&target_dir.join("dto.rs"), &dto_content);

    // Write service.rs
    let service_content = format!(
        r#"use sea_orm::{{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set,
}};
use crate::{{
    db::models::{SnakeName} as {SnakeName}_model,
    infra::error::AppError,
}};

pub struct {CamelName}Service;

impl {CamelName}Service {{
    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
    ) -> Result<{SnakeName}_model::Model, AppError> {{
        let active_model = {SnakeName}_model::ActiveModel {{
            name: Set(name.to_owned()),
            ..Default::default()
        }};
        let model = active_model.insert(db).await?;
        Ok(model)
    }}

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<{SnakeName}_model::Model>, AppError> {{
        let model = {SnakeName}_model::Entity::find_by_id(id).one(db).await?;
        Ok(model)
    }}

    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<{SnakeName}_model::Model>, u64), AppError> {{
        let paginator = {SnakeName}_model::Entity::find()
            .order_by_desc({SnakeName}_model::Column::Id)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        Ok((items, total))
    }}

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        name: &str,
    ) -> Result<{SnakeName}_model::Model, AppError> {{
        let model = {SnakeName}_model::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("{{}} not found", "{CamelName}")))?;

        let mut active_model: {SnakeName}_model::ActiveModel = model.into();
        active_model.name = Set(name.to_owned());
        active_model.updated_at = Set(chrono::DateTime::<chrono::FixedOffset>::from(chrono::Utc::now()));

        let updated = active_model.update(db).await?;
        Ok(updated)
    }}

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {{
        let result = {SnakeName}_model::Entity::delete_by_id(id).exec(db).await?;
        if result.rows_affected == 0 {{
            return Err(AppError::NotFound(format!("{{}} not found", "{CamelName}")));
        }}
        Ok(())
    }}
}}
"#,
        CamelName = camel_case,
        SnakeName = feature_name
    );
    let _ = write_file(&target_dir.join("service.rs"), &service_content);

    // Write handler.rs
    let handler_content = format!(
        r#"use axum::{{
    extract::{{Path, Query, State}},
    http::StatusCode,
    Json,
}};
use sea_orm::DatabaseConnection;

use crate::{{
    features::{SnakeName}::{{
        dto::{{Create{CamelName}Request, Update{CamelName}Request, {CamelName}Response}},
        service::{CamelName}Service,
    }},
    infra::{{
        error::{{AppError, ErrorResponse}},
        extractor::ValidatedJson,
        pagination::{{PaginatedResponse, PaginationParams}},
    }},
}};

#[utoipa::path(
    post,
    path = "/api/{KebabName}",
    request_body = Create{CamelName}Request,
    responses(
        (status = 201, description = "Created successfully", body = {CamelName}Response),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 409, description = "Conflict", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn create(
    State(db): State<DatabaseConnection>,
    ValidatedJson(payload): ValidatedJson<Create{CamelName}Request>,
) -> Result<(StatusCode, Json<{CamelName}Response>), AppError> {{
    let result = {CamelName}Service::create(&db, &payload.name).await?;
    Ok((StatusCode::CREATED, Json({CamelName}Response::from(result))))
}}

#[utoipa::path(
    get,
    path = "/api/{KebabName}",
    params(PaginationParams),
    responses(
        (status = 200, description = "List retrieved successfully", body = PaginatedResponse<{CamelName}Response>),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn list(
    State(db): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>,
) -> Result<(StatusCode, Json<PaginatedResponse<{CamelName}Response>>), AppError> {{
    let page = params.page();
    let per_page = params.per_page();
    let (items, total) = {CamelName}Service::list(&db, page, per_page).await?;
    let data = items.into_iter().map({CamelName}Response::from).collect();
    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::new(data, page, per_page, total)),
    ))
}}

#[utoipa::path(
    get,
    path = "/api/{KebabName}/{{id}}",
    params(
        ("id" = i32, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Retrieved successfully", body = {CamelName}Response),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<{CamelName}Response>), AppError> {{
    let model = {CamelName}Service::find_by_id(&db, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("{{}} not found", "{CamelName}")))?;
    Ok((StatusCode::OK, Json({CamelName}Response::from(model))))
}}

#[utoipa::path(
    put,
    path = "/api/{KebabName}/{{id}}",
    request_body = Update{CamelName}Request,
    params(
        ("id" = i32, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Updated successfully", body = {CamelName}Response),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn update(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<Update{CamelName}Request>,
) -> Result<(StatusCode, Json<{CamelName}Response>), AppError> {{
    let result = {CamelName}Service::update(&db, id, &payload.name).await?;
    Ok((StatusCode::OK, Json({CamelName}Response::from(result))))
}}

#[utoipa::path(
    delete,
    path = "/api/{KebabName}/{{id}}",
    params(
        ("id" = i32, Path, description = "Resource ID")
    ),
    responses(
        (status = 204, description = "Deleted successfully"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn delete(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {{
    {CamelName}Service::delete(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}}
"#,
        CamelName = camel_case,
        SnakeName = feature_name,
        KebabName = kebab_case
    );
    let _ = write_file(&target_dir.join("handler.rs"), &handler_content);

    // Write src/routes/{name}.rs
    let routes_file_path = format!("src/routes/{}.rs", feature_name);
    let routes_content = format!(
        r#"use axum::{{
    routing::get,
    Router,
}};

use crate::{{features::{SnakeName}::handler as {SnakeName}_handler, routes::AppState}};

pub fn router() -> Router<AppState> {{
    Router::new()
        .route("/", get({SnakeName}_handler::list).post({SnakeName}_handler::create))
        .route(
            "/{{id}}",
            get({SnakeName}_handler::get_by_id)
                .put({SnakeName}_handler::update)
                .delete({SnakeName}_handler::delete),
        )
}}
"#,
        SnakeName = feature_name
    );
    let _ = write_file(Path::new(&routes_file_path), &routes_content);

    register_feature_in_mod(feature_name);
    register_routes_in_mod(feature_name, &kebab_case);
    register_in_swagger(feature_name, &camel_case);

    println!("Resource '{}' generated successfully!", feature_name);
}

fn register_feature_in_mod(feature_name: &str) {
    let mod_file_path = Path::new("src/features/mod.rs");
    if mod_file_path.exists() {
        let register_line = format!("pub mod {};", feature_name);
        if let Ok(content) = fs::read_to_string(mod_file_path) {
            if !content.contains(&register_line) {
                println!("Registering module in {}...", mod_file_path.display());
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(mod_file_path)
                    .expect("Failed to open features mod.rs for appending");
                if let Err(e) = writeln!(file, "pub mod {};", feature_name) {
                    eprintln!("Failed to append module registration: {}", e);
                }
            }
        }
    }
}

fn register_routes_in_mod(feature_name: &str, kebab_case: &str) {
    let mod_file_path = Path::new("src/routes/mod.rs");
    if !mod_file_path.exists() {
        return;
    }

    if let Ok(mut content) = fs::read_to_string(mod_file_path) {
        let mod_decl = format!("pub mod {};", feature_name);
        if !content.contains(&mod_decl) {
            // Find a good place to insert "pub mod name;"
            if let Some(pos) = content.find("pub mod health;") {
                content.insert_str(pos, &format!("pub mod {};\n", feature_name));
            } else {
                content.push_str(&format!("\npub mod {};\n", feature_name));
            }
        }

        let nest_decl = format!(".nest(\"/{}\", {}::router())", kebab_case, feature_name);
        if !content.contains(&nest_decl) {
            if let Some(pos) = content.find(".nest(\"/health\"") {
                content.insert_str(pos, &format!(".nest(\"/{}\", {}::router())\n        ", kebab_case, feature_name));
            }
        }

        if let Err(e) = fs::write(mod_file_path, content) {
            eprintln!("Failed to auto-register routes in mod.rs: {}", e);
        }
    }
}

fn register_in_swagger(feature_name: &str, camel_case: &str) {
    let swagger_file_path = Path::new("src/routes/swagger.rs");
    if !swagger_file_path.exists() {
        return;
    }

    if let Ok(mut content) = fs::read_to_string(swagger_file_path) {
        // Paths
        let paths_marker = "paths(";
        if let Some(pos) = content.find(paths_marker) {
            let insert_pos = pos + paths_marker.len();
            let new_paths = format!(
                "\n        crate::features::{SnakeName}::handler::create,\n        crate::features::{SnakeName}::handler::list,\n        crate::features::{SnakeName}::handler::get_by_id,\n        crate::features::{SnakeName}::handler::update,\n        crate::features::{SnakeName}::handler::delete,",
                SnakeName = feature_name
            );
            if !content.contains(&format!("crate::features::{}::handler::create", feature_name)) {
                content.insert_str(insert_pos, &new_paths);
            }
        }

        // Schemas
        let schemas_marker = "components(schemas(";
        if let Some(pos) = content.find(schemas_marker) {
            let insert_pos = pos + schemas_marker.len();
            let new_schemas = format!(
                "\n        crate::features::{SnakeName}::dto::Create{CamelName}Request,\n        crate::features::{SnakeName}::dto::Update{CamelName}Request,\n        crate::features::{SnakeName}::dto::{CamelName}Response,\n        crate::infra::pagination::PaginatedResponse<crate::features::{SnakeName}::dto::{CamelName}Response>,",
                SnakeName = feature_name,
                CamelName = camel_case
            );
            if !content.contains(&format!("crate::features::{}::dto::Create{}Request", feature_name, camel_case)) {
                content.insert_str(insert_pos, &new_schemas);
            }
        }

        if let Err(e) = fs::write(swagger_file_path, content) {
            eprintln!("Failed to register components in swagger.rs: {}", e);
        }
    }
}

fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn is_valid_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if !first.is_ascii_lowercase() {
            return false;
        }
    }
    for c in chars {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '_' {
            return false;
        }
    }
    true
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;
    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn to_kebab_case(s: &str) -> String {
    s.replace('_', "-")
}
