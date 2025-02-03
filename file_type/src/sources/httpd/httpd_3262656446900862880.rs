use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3262656446900862880: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "chat",
    extensions: &["chat"],
    media_types: &["application/x-chat"],
    internal_signatures: &[],
    related_formats: &[],
};
