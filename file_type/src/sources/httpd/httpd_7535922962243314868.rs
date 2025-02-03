use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7535922962243314868: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msclip",
    extensions: &["clp"],
    media_types: &["application/x-msclip"],
    internal_signatures: &[],
    related_formats: &[],
};
