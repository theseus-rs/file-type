use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3824116529: FileFormat = FileFormat {
    id: 3_824_116_529,
    source_type: SourceType::Iana,
    name: "vnd.proteus.magazine",
    extensions: &[],
    media_types: &["application/vnd.proteus.magazine"],
    signatures: &[],
    related_formats: &[],
};
