use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14576682968901447606: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "html",
    extensions: &["html", "htm"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
