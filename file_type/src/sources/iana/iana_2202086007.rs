use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2202086007: FileFormat = FileFormat {
    id: 2_202_086_007,
    source_type: SourceType::Iana,
    name: "jpx",
    extensions: &[],
    media_types: &["image/jpx"],
    signatures: &[],
    related_formats: &[],
};
