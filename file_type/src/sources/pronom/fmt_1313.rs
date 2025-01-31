use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1313: FileFormat = FileFormat {
    id: 2_131,
    puid: "fmt/1313",
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
                        0x43, 0x6F, 0x72, 0x65, 0x6C, 0x43, 0x48, 0x41, 0x52, 0x54, 0x20, 0x56,
                        0x35,
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
