use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3013817452668215055: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mp4",
    extensions: &["mp4s"],
    media_types: &["application/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
