use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1462109934: FileFormat = FileFormat {
    id: 1_462_109_934,
    source_type: SourceType::Iana,
    name: "mbox",
    extensions: &[],
    media_types: &["application/mbox"],
    internal_signatures: &[],
    related_formats: &[],
};
