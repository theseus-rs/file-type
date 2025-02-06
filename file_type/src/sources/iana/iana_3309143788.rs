use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3309143788: FileFormat = FileFormat {
    id: 3_309_143_788,
    source_type: SourceType::Iana,
    name: "vnd.apache.thrift.binary",
    extensions: &[],
    media_types: &["application/vnd.apache.thrift.binary"],
    signatures: &[],
    related_formats: &[],
};
