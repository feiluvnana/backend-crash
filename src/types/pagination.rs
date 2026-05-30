use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, Clone)]
pub struct PaginationParams {
    #[param(default = 1)]
    pub page: Option<u64>,
    #[param(default = 20)]
    pub per_page: Option<u64>,
}

impl PaginationParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct PageMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u64, per_page: u64, total: u64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as u64;
        Self {
            data,
            meta: PageMeta {
                page,
                per_page,
                total,
                total_pages,
            },
        }
    }
}
