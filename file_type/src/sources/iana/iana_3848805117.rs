use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3848805117: FileFormat = FileFormat {
    id: 3_848_805_117,
    source_type: SourceType::Iana,
    name: "vnd.acm.addressxfer+json",
    extensions: &[],
    media_types: &["application/vnd.acm.addressxfer+json"],
    signatures: &[],
    related_formats: &[],
};
