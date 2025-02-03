use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_111619794799683331: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "asm",
    extensions: &["s", "asm"],
    media_types: &["text/x-asm"],
    internal_signatures: &[],
    related_formats: &[],
};
