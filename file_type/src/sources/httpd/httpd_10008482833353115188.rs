use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10008482833353115188: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "wolfram player",
    extensions: &["nbp"],
    media_types: &["application/vnd.wolfram.player"],
    internal_signatures: &[],
    related_formats: &[],
};
