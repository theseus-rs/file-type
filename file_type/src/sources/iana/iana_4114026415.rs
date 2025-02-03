use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4114026415: FileFormat = FileFormat {
    id: 4_114_026_415,
    source_type: SourceType::Iana,
    name: "vnd.apache.thrift.compact",
    extensions: &[],
    media_types: &["application/vnd.apache.thrift.compact"],
    internal_signatures: &[],
    related_formats: &[],
};
