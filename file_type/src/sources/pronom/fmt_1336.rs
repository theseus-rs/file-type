use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1336: FileFormat = FileFormat {
    id: 2_154,
    puid: "fmt/1336",
    name: "LEADTools Lead 1Bit Compressed Image",
    extensions: &["cmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x54, 0x52, 0x49, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
