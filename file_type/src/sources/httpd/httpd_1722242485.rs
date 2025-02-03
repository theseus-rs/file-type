use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1722242485: FileFormat = FileFormat {
    id: 1_722_242_485,
    source_type: SourceType::Httpd,
    name: "google earth kmz",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    internal_signatures: &[],
    related_formats: &[],
};
