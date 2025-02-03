use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4486161733139916318: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msmetafile",
    extensions: &["wmf", "wmz", "emf", "emz"],
    media_types: &["application/x-msmetafile"],
    internal_signatures: &[],
    related_formats: &[],
};
