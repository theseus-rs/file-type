use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1739691841: FileFormat = FileFormat {
    id: 1_739_691_841,
    source_type: SourceType::Iana,
    name: "zstd",
    extensions: &[],
    media_types: &["application/zstd"],
    signatures: &[],
    related_formats: &[],
};
