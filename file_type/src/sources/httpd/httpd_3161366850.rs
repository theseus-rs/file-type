use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3161366850: FileFormat = FileFormat {
    id: 3_161_366_850,
    source_type: SourceType::Httpd,
    name: "ms lrm",
    extensions: &["lrm"],
    media_types: &["application/vnd.ms-lrm"],
    internal_signatures: &[],
    related_formats: &[],
};
