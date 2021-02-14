use serde::Deserialize;
use super::http::{get, post};
use super::api::{v3, AccessToken};
use super::error::Result;
use reqwest::multipart::{Form, Part};

#[derive(Debug)]
pub enum DataType {
    Fit, FitGz, Tcx, TcxGz, Gpx, GpxGz
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        let s = match self {
            DataType::Fit => "fit",
            DataType::FitGz => "fit_gz",
            DataType::Tcx => "tcx",
            DataType::TcxGz => "tcx_gz",
            DataType::Gpx => "gpx",
            DataType::GpxGz => "gpx_gz",
        };
        s.to_string()
    }
}

#[derive(Debug)]
pub struct CreateUpload {
    pub name : Option<String>,
    pub description : Option<String>,
    pub trainer : Option<bool>,
    pub commute: Option<bool>,
    pub data_type : DataType,
    pub external_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Upload {
    pub id_str : Option<String>,
    pub activity_id : Option<usize>,
    pub external_id : Option<String>,
    pub id: Option<usize>,
    pub error : Option<String>,
    pub status : Option<String>,
}

impl Upload {
    pub async fn get(token: &AccessToken, upload_id : u32) -> Result<Self> {
        let url = v3(Some(token), format!("uploads/{}", upload_id));
        Ok(get::<Upload>(&url[..]).await?)
    }

    pub async fn upload(token: &AccessToken, upload : CreateUpload, data : String) -> Result<Self> {
        let url = v3(Some(token), "uploads".to_string());

        let mut form = Form::new()
            .text("data_type", upload.data_type.to_string())
            .text("external_id", upload.external_id)
            .part("file", Part::text(data).file_name("file"));

        if upload.name.is_some() {
            form = form.text("name", upload.name.unwrap());
        }
        if upload.description.is_some() {
            form = form.text("description", upload.description.unwrap());
        }
        if upload.trainer.is_some() {
            let val = if upload.trainer.unwrap() {
                "1"
            } else {
                "0"
            };
            form = form.text("trainer", val);
        }
        if upload.commute.is_some() {
            let val = if upload.commute.unwrap() {
                "1"
            } else {
                "0"
            };
            form = form.text("commute", val);
        }
        post::<Upload>(&url[..], form).await
    }
}
