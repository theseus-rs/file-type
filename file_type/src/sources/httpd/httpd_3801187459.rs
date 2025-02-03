use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3801187459: FileFormat = FileFormat {
    id: 3_801_187_459,
    source_type: SourceType::Httpd,
    name: "mobius mqy",
    extensions: &["mqy"],
    media_types: &["application/vnd.mobius.mqy"],
    internal_signatures: &[],
    related_formats: &[],
};
