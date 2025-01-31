use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1483: FileFormat = FileFormat {
    id: 2_306,
    puid: "fmt/1483",
    name: "Mar Archive",
    extensions: &["mar", "mac"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x52, 0x80])],
            },
        }],
    }],
    related_formats: &[],
};
