use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2519989976: FileFormat = FileFormat {
    id: 2_519_989_976,
    source_type: SourceType::Iana,
    name: "zlib",
    extensions: &[],
    media_types: &["application/zlib"],
    signatures: &[],
    related_formats: &[],
};
