use dioxus::prelude::*;

#[derive(Clone)]
pub struct BrandContext {
    pub name: String,
    pub logo: Asset,
    pub primary_color: String,
    pub secondary_color: String,
}
