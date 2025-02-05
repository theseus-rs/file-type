use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1432736077: FileFormat = FileFormat {
    id: 1_432_736_077,
    source_type: SourceType::Httpd,
    name: "matroska",
    extensions: &["mka"],
    media_types: &["audio/x-matroska"],
    signatures: &[],
    related_formats: &[],
};
