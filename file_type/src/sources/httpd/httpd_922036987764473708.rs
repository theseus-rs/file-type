use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_922036987764473708: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "google earth kml xml",
    extensions: &["kml"],
    media_types: &["application/vnd.google-earth.kml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
