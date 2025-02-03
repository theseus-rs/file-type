use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13249549495329969252: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mads xml",
    extensions: &["mads"],
    media_types: &["application/mads+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
