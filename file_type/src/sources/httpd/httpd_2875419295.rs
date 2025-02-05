use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2875419295: FileFormat = FileFormat {
    id: 2_875_419_295,
    source_type: SourceType::Httpd,
    name: "asm",
    extensions: &["s", "asm"],
    media_types: &["text/x-asm"],
    signatures: &[],
    related_formats: &[],
};
