use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1504920749: FileFormat = FileFormat {
    id: 1_504_920_749,
    source_type: SourceType::Httpd,
    name: "google earth kml xml",
    extensions: &["kml"],
    media_types: &["application/vnd.google-earth.kml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
