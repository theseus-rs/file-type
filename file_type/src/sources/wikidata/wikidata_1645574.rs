use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1645574: FileFormat = FileFormat {
    id: 1_645_574,
    puid: "wikidata/1645574",
    name: "Protocol Buffers",
    extensions: &["binpb", "binpb", "proto", "proto", "txtpb", "txtpb"],
    media_types: &[
        "application/protobuf",
        "application/protobuf",
        "application/protobuf",
        "application/vnd.google.protobuf",
        "application/vnd.google.protobuf",
        "application/vnd.google.protobuf",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
