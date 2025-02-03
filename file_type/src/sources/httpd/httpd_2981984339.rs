use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2981984339: FileFormat = FileFormat {
    id: 2_981_984_339,
    source_type: SourceType::Httpd,
    name: "powerbuilder6",
    extensions: &["pbd"],
    media_types: &["application/vnd.powerbuilder6"],
    internal_signatures: &[],
    related_formats: &[],
};
