use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17719390468068402583: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "zmachine",
    extensions: &["z1", "z2", "z3", "z4", "z5", "z6", "z7", "z8"],
    media_types: &["application/x-zmachine"],
    internal_signatures: &[],
    related_formats: &[],
};
