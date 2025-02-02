use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13373727545469518801: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "clonk c4group",
    extensions: &["c4g", "c4d", "c4f", "c4p", "c4u"],
    media_types: &["application/vnd.clonk.c4group"],
    internal_signatures: &[],
    related_formats: &[],
};
