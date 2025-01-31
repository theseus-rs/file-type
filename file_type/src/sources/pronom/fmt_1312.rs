use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1312: FileFormat = FileFormat {
    id: 2_130,
    puid: "fmt/1312",
    name: "CorelCHART Document",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[
                        0x33, 0x44, 0x46, 0x2E, 0x30, 0x30, 0x30, 0x32, 0x20, 0x30, 0x34, 0x64,
                        0x65, 0x63, 0x39, 0x31,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_099,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
