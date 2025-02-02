use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2303: FileFormat = FileFormat {
    id: 2_303,
    source_type: SourceType::Pronom,
    name: "XIFF (Xerox Image File Format)",
    extensions: &["xif"],
    media_types: &["image/vnd.xiff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[
                        0x20, 0x65, 0x58, 0x74, 0x65, 0x6E, 0x64, 0x65, 0x64, 0x20, 0x03,
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
