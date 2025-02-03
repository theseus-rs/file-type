use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3519971946: FileFormat = FileFormat {
    id: 3_519_971_946,
    source_type: SourceType::Iana,
    name: "mmt-aei+xml",
    extensions: &[],
    media_types: &["application/mmt-aei+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
