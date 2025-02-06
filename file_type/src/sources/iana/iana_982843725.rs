use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_982843725: FileFormat = FileFormat {
    id: 982_843_725,
    source_type: SourceType::Iana,
    name: "rdf+xml",
    extensions: &[],
    media_types: &["application/rdf+xml"],
    signatures: &[],
    related_formats: &[],
};
