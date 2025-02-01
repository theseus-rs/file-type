use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1276: FileFormat = FileFormat {
    id: 2_094,
    puid: "fmt/1276",
    name: "SureThing Project File",
    extensions: &["std"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x56, 0x00, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
