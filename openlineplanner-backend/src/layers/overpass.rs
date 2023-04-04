use std::collections::HashMap;

use geo::Point;
use serde::Deserialize;

use anyhow::Result;

#[derive(Deserialize)]
pub struct OverpassResponse {
    version: f32,
    generator: String,
    pub elements: Vec<OverpassResponseElement>,
}

#[derive(Deserialize)]
pub struct OverpassResponseElement {
    pub ovp_type: String,
    pub id: u64,
    pub tags: HashMap<String, String>,
    pub bounds: OverpassElementBounds,
    pub members: Vec<OverpassGeometryElement>,
}

#[derive(Deserialize)]
pub struct OverpassElementBounds {
    minlat: f64,
    minlon: f64,
    maxlat: f64,
    maxlon: f64,
}

impl From<OverpassElementBounds> for Vec<f64> {
    fn from(value: OverpassElementBounds) -> Self {
        vec![value.maxlat, value.maxlon, value.minlat, value.minlon]
    }
}

#[derive(Deserialize)]
pub struct OverpassGeometryElement {
    pub ovp_type: String,
    pub role: String,
    pub ovp_ref: u64,
    pub geometry: Vec<OverpassGeometryPoint>,
}

#[derive(Deserialize)]
pub struct OverpassGeometryPoint {
    lat: f64,
    lon: f64,
}

impl From<OverpassGeometryPoint> for Point {
    fn from(value: OverpassGeometryPoint) -> Self {
        Self::new(value.lon, value.lat)
    }
}

pub async fn query_overpass(query: String) -> Result<OverpassResponse> {
    let client = reqwest::Client::new();
    Ok(client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()
        .await?
        .json::<OverpassResponse>()
        .await?)
}
