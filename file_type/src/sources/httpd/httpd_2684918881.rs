use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2684918881: FileFormat = FileFormat {
    id: 2_684_918_881,
    source_type: SourceType::Httpd,
    name: "collada xml",
    extensions: &["dae"],
    media_types: &["model/vnd.collada+xml"],
    signatures: &[],
    related_formats: &[],
};
