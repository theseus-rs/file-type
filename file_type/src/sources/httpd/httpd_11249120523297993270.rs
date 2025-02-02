use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11249120523297993270: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "google earth kmz",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    internal_signatures: &[],
    related_formats: &[],
};
