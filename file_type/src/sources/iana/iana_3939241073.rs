use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3939241073: FileFormat = FileFormat {
    id: 3_939_241_073,
    source_type: SourceType::Iana,
    name: "vnd.lotus-wordpro",
    extensions: &[],
    media_types: &["application/vnd.lotus-wordpro"],
    signatures: &[],
    related_formats: &[],
};
