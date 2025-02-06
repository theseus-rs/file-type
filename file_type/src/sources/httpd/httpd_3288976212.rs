use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3288976212: FileFormat = FileFormat {
    id: 3_288_976_212,
    source_type: SourceType::Httpd,
    name: "pmi widget",
    extensions: &["wg"],
    media_types: &["application/vnd.pmi.widget"],
    signatures: &[],
    related_formats: &[],
};
