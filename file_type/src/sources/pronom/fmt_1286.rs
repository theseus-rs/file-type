use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1286: FileFormat = FileFormat {
    id: 2_104,
    puid: "fmt/1286",
    name: "Envoy Document File",
    extensions: &["evy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x5E, 0x10, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
