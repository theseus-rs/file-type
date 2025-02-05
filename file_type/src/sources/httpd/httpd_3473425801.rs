use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3473425801: FileFormat = FileFormat {
    id: 3_473_425_801,
    source_type: SourceType::Httpd,
    name: "clonk c4group",
    extensions: &["c4g", "c4d", "c4f", "c4p", "c4u"],
    media_types: &["application/vnd.clonk.c4group"],
    signatures: &[],
    related_formats: &[],
};
