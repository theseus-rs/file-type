use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1442136583: FileFormat = FileFormat {
    id: 1_442_136_583,
    source_type: SourceType::Iana,
    name: "vnd.patentdive",
    extensions: &[],
    media_types: &["application/vnd.patentdive"],
    signatures: &[],
    related_formats: &[],
};
