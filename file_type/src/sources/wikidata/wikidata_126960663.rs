use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126960663: FileFormat = FileFormat {
    id: 126_960_663,
    puid: "wikidata/126960663",
    name: "Apache Thrift file",
    extensions: &["thrift"],
    media_types: &["application/x-thrift"],
    internal_signatures: &[],
    related_formats: &[],
};
