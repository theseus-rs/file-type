use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3718262307: FileFormat = FileFormat {
    id: 3_718_262_307,
    source_type: SourceType::Httpd,
    name: "s3m",
    extensions: &["s3m"],
    media_types: &["audio/s3m"],
    signatures: &[],
    related_formats: &[],
};
