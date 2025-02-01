use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1287: FileFormat = FileFormat {
    id: 2_105,
    puid: "fmt/1287",
    name: "Envoy Document File",
    extensions: &["evy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB2, 0x97, 0xE1, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
