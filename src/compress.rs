//! Response compression module.

use std::path::Path;
use std::io::{self, Cursor, Write};

use failure::Error;
use rocket::{Request, Response};
use rocket::response::{NamedFile, Responder};
use rocket::http::{ContentType, Status};
use rocket_contrib::Template;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::Serialize;

/// Compressed template.
#[derive(Debug)]
pub struct CompressedTemplate {
    /// Template to compress.
    template: Template,
}

impl CompressedTemplate {
    /// Creates a new compressed template from a template.
    pub fn new(template: Template) -> CompressedTemplate {
        CompressedTemplate { template }
    }
}

impl<'r> Responder<'r> for CompressedTemplate {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        let mut response = self.template.respond_to(request)?;

        let headers = request.headers();
        // Check if requests accepts gzip encoding.
        if headers.contains("Accept")
            && headers.get("Accept-Encoding").any(
                |e| e.to_lowercase() == "gzip",
                // We compress the response here.
            ) && compress_response(&mut response).is_err()
        {
            // Return an internal server error if compression went wrong.
            return Err(Status::InternalServerError);
        }
        Ok(response)
    }
}

/// Compressed file. Do not use for images.
#[derive(Debug)]
pub struct CompressedFile {
    /// File to compress.
    file: NamedFile,
    /// Content type for the response.
    content_type: ContentType,
}

impl CompressedFile {
    /// Creates a new compressed file from a path and a content type.
    pub fn new<P: AsRef<Path>>(
        file: P,
        content_type: ContentType,
    ) -> Result<CompressedFile, Error> {
        Ok(CompressedFile {
            file: NamedFile::open(file)?,
            content_type,
        })
    }
}

impl<'r> Responder<'r> for CompressedFile {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        let mut response = self.file.respond_to(request)?;
        let _ = response.set_header(self.content_type);

        let headers = request.headers();
        // Check if requests accepts gzip encoding.
        if headers.contains("Accept")
            && headers.get("Accept-Encoding").any(
                |e| e.to_lowercase() == "gzip",
                // We compress the response here.
            ) && compress_response(&mut response).is_err()
        {
            // Return an internal server error if compression went wrong.
            return Err(Status::InternalServerError);
        }
        Ok(response)
    }
}

/// Compressed JSON response.
#[derive(Debug)]
pub struct CompressedJson<T> {
    /// JSON data to compress.
    data: T,
}

impl<T> CompressedJson<T> {
    /// Creates a new compressed JSON from the given data.
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<'r, T> Responder<'r> for CompressedJson<T>
where
    T: Serialize,
{
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        use rocket_contrib::Json;

        let mut response = Json(self.data).respond_to(request)?;

        let headers = request.headers();
        // Check if requests accepts gzip encoding.
        if headers.contains("Accept")
            && headers.get("Accept-Encoding").any(
                |e| e.to_lowercase() == "gzip",
                // We compress the response here.
            ) && compress_response(&mut response).is_err()
        {
            // Return an internal server error if compression went wrong.
            return Err(Status::InternalServerError);
        }
        Ok(response)
    }
}

/// Compresses the given response using Gzip.
///
/// Note that you should check if the client accepts compressed responses before compressing it.
fn compress_response(response: &mut Response) -> Result<(), Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&response.body_bytes().unwrap_or_default())?;

    let _ = response.set_raw_header("Content-Encoding", "gzip");
    response.set_sized_body(Cursor::new(encoder.finish()?));

    Ok(())
}
