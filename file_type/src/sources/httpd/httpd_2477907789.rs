use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2477907789: FileFormat = FileFormat {
    id: 2_477_907_789,
    source_type: SourceType::Httpd,
    name: "lotus notes",
    extensions: &["nsf"],
    media_types: &["application/vnd.lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
