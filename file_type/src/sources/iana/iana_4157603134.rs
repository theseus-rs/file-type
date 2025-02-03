use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4157603134: FileFormat = FileFormat {
    id: 4_157_603_134,
    source_type: SourceType::Iana,
    name: "PCMU",
    extensions: &[],
    media_types: &["audio/PCMU"],
    internal_signatures: &[],
    related_formats: &[],
};
