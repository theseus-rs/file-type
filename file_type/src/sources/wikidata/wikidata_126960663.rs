use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126960663: FileFormat = FileFormat {
    id: 126_960_663,
    source_type: SourceType::Wikidata,
    name: "Apache Thrift file",
    extensions: &["thrift"],
    media_types: &["application/x-thrift"],
    signatures: &[],
    related_formats: &[],
};
