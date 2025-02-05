use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1623729236: FileFormat = FileFormat {
    id: 1_623_729_236,
    source_type: SourceType::Httpd,
    name: "lotus 1 2 3",
    extensions: &["123"],
    media_types: &["application/vnd.lotus-1-2-3"],
    signatures: &[],
    related_formats: &[],
};
