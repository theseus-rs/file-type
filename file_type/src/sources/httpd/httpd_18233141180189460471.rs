use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18233141180189460471: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ufdl",
    extensions: &["ufd", "ufdl"],
    media_types: &["application/vnd.ufdl"],
    internal_signatures: &[],
    related_formats: &[],
};
