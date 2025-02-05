use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1209759489: FileFormat = FileFormat {
    id: 1_209_759_489,
    source_type: SourceType::Iana,
    name: "sbe",
    extensions: &[],
    media_types: &["application/sbe"],
    signatures: &[],
    related_formats: &[],
};
