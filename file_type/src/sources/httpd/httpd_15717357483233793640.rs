use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15717357483233793640: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hal xml",
    extensions: &["hal"],
    media_types: &["application/vnd.hal+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
