use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3160205525: FileFormat = FileFormat {
    id: 3_160_205_525,
    source_type: SourceType::Iana,
    name: "index",
    extensions: &[],
    media_types: &["application/index"],
    signatures: &[],
    related_formats: &[],
};
