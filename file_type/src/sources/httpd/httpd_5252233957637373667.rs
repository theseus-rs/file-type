use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5252233957637373667: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rfc822",
    extensions: &["eml", "mime"],
    media_types: &["message/rfc822"],
    internal_signatures: &[],
    related_formats: &[],
};
