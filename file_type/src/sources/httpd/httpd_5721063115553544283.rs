use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5721063115553544283: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fly",
    extensions: &["fly"],
    media_types: &["text/vnd.fly"],
    internal_signatures: &[],
    related_formats: &[],
};
