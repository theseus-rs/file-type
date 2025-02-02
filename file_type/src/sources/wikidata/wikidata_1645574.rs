use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1645574: FileFormat = FileFormat {
    id: 1_645_574,
    source_type: SourceType::Wikidata,
    name: "Protocol Buffers",
    extensions: &["binpb", "proto", "txtpb"],
    media_types: &["application/protobuf", "application/vnd.google.protobuf"],
    internal_signatures: &[],
    related_formats: &[],
};
