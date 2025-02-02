use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7152264101637064257: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "vcard",
    extensions: &["vcard"],
    media_types: &["text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
