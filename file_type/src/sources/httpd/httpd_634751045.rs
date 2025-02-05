use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_634751045: FileFormat = FileFormat {
    id: 634_751_045,
    source_type: SourceType::Httpd,
    name: "cosmocaller",
    extensions: &["cmc"],
    media_types: &["application/vnd.cosmocaller"],
    signatures: &[],
    related_formats: &[],
};
