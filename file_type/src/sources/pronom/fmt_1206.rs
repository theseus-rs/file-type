use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1206: FileFormat = FileFormat {
    id: 2_016,
    puid: "fmt/1206",
    name: "Impulse 3D Data Description Object",
    extensions: &["iob"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x54, 0x44, 0x44, 0x44]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 221,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
