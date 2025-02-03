use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11028599860578863387: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "latex",
    extensions: &["latex"],
    media_types: &["application/x-latex"],
    internal_signatures: &[],
    related_formats: &[],
};
