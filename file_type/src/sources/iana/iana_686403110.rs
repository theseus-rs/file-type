use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_686403110: FileFormat = FileFormat {
    id: 686_403_110,
    source_type: SourceType::Iana,
    name: "sofa",
    extensions: &[],
    media_types: &["audio/sofa"],
    internal_signatures: &[],
    related_formats: &[],
};
