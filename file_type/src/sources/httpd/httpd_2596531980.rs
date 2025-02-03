use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2596531980: FileFormat = FileFormat {
    id: 2_596_531_980,
    source_type: SourceType::Httpd,
    name: "rgb",
    extensions: &["rgb"],
    media_types: &["image/x-rgb"],
    internal_signatures: &[],
    related_formats: &[],
};
