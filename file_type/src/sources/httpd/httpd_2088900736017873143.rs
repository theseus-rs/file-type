use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2088900736017873143: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "atomcat xml",
    extensions: &["atomcat"],
    media_types: &["application/atomcat+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
