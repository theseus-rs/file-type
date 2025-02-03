use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7535621132484556555: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "epub zip",
    extensions: &["epub"],
    media_types: &["application/epub+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
