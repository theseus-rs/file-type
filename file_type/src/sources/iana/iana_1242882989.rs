use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1242882989: FileFormat = FileFormat {
    id: 1_242_882_989,
    source_type: SourceType::Iana,
    name: "jose+json",
    extensions: &[],
    media_types: &["application/jose+json"],
    signatures: &[],
    related_formats: &[],
};
