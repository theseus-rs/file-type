use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1596643479: FileFormat = FileFormat {
    id: 1_596_643_479,
    source_type: SourceType::Iana,
    name: "GSM-EFR",
    extensions: &[],
    media_types: &["audio/GSM-EFR"],
    signatures: &[],
    related_formats: &[],
};
