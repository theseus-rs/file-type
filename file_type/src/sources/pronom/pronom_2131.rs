use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2131: FileFormat = FileFormat {
    id: 2_131,
    source_type: SourceType::Pronom,
    name: "CorelCHART Document",
    extensions: &["cch"],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_099,
    }],
};
