use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13615448736560673113: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "onenote",
    extensions: &["onetoc", "onetoc2", "onetmp", "onepkg"],
    media_types: &["application/onenote"],
    internal_signatures: &[],
    related_formats: &[],
};
