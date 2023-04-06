

use bytes::{buf::Reader, Buf, Bytes};

use anyhow::Result;
use osmpbfreader::OsmPbfReader;
use serde::{Deserialize, Serialize};

use acc_reader::AccReader;

use super::osm::AdminArea;

#[derive(Serialize)]
struct ProtomapsDownloadRequest {
    name: String,
    region: ProtomapsDownloadRegion,
}

#[derive(Serialize)]
struct ProtomapsDownloadRegion {
    data: Vec<f64>,
    #[serde(rename = "type")]
    region_type: String,
}

impl ProtomapsDownloadRegion {
    pub fn from_admin_area(admin_area: AdminArea) -> Self {
        Self {
            data: admin_area.bounding_box,
            region_type: "bbox".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct ProtomapsAreaRequest {
    uuid: String,
    url: String,
}

impl ProtomapsAreaRequest {
    async fn ready(&self, client: &reqwest::Client) -> Result<ProtomapsDownload> {
        let resp = client
            .get(&self.url)
            .header("Referer", "https://app.protomaps.com/")
            .send()
            .await?;
        Ok(resp.json().await?)
    }
    async fn wait_until_ready(self, client: &reqwest::Client) -> Result<ProtomapsDownload> {
        loop {
            let download = self.ready(client).await?;
            if download.complete.unwrap_or(false) {
                return Ok(download);
            }
        }
    }
}

#[derive(Deserialize)]
struct ProtomapsDownload {
    uuid: Option<String>,
    complete: Option<bool>,
}

impl ProtomapsDownload {
    async fn download(self, client: &reqwest::Client) -> Result<Bytes> {
        let response = client
            .get(format!(
                "https://app.protomaps.com/downloads/{}/download",
                self.uuid
                    .ok_or(anyhow::anyhow!("did not get valid uuid for download"))?
            ))
            .header("Referer", "https://app.protomaps.com/")
            .send()
            .await?;
        Ok(response.bytes().await?)
    }
}

pub async fn download_pbf(admin_area: AdminArea) -> Result<OsmPbfReader<AccReader<Reader<Bytes>>>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let resp = client.get("https://app.protomaps.com/downloads/osm").send().await;

    let request = ProtomapsDownloadRequest {
        name: "".to_string(),
        region: ProtomapsDownloadRegion::from_admin_area(admin_area),
    };

    let area_req = client
        .post("https://app.protomaps.com/downloads/osm")
        .header("Referer", "https://app.protomaps.com/")
        .header("Origin", "https://app.protomaps.com")
        .json(&request)
        .send()
        .await?
        .text()
        .await?;
    println!("{}", area_req);
    let pbf = serde_json::from_str::<ProtomapsAreaRequest>(&area_req)?
        .wait_until_ready(&client)
        .await?
        .download(&client)
        .await?;

    let pbf_reader = OsmPbfReader::new(AccReader::new(pbf.reader()));
    Ok(pbf_reader)
}
