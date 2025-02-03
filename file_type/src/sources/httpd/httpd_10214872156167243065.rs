use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10214872156167243065: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rtf",
    extensions: &["rtf"],
    media_types: &["application/rtf"],
    internal_signatures: &[],
    related_formats: &[],
};
