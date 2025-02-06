use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_676122532: FileFormat = FileFormat {
    id: 676_122_532,
    source_type: SourceType::Iana,
    name: "jaii",
    extensions: &[],
    media_types: &["image/jaii"],
    signatures: &[],
    related_formats: &[],
};
