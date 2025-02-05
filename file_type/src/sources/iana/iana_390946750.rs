use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_390946750: FileFormat = FileFormat {
    id: 390_946_750,
    source_type: SourceType::Iana,
    name: "load-control+xml",
    extensions: &[],
    media_types: &["application/load-control+xml"],
    signatures: &[],
    related_formats: &[],
};
