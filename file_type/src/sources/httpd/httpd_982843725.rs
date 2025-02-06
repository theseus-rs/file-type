use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_982843725: FileFormat = FileFormat {
    id: 982_843_725,
    source_type: SourceType::Httpd,
    name: "rdf xml",
    extensions: &["rdf"],
    media_types: &["application/rdf+xml"],
    signatures: &[],
    related_formats: &[],
};
