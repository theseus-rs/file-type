use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4266494892: FileFormat = FileFormat {
    id: 4_266_494_892,
    source_type: SourceType::Httpd,
    name: "rls services xml",
    extensions: &["rs"],
    media_types: &["application/rls-services+xml"],
    signatures: &[],
    related_formats: &[],
};
