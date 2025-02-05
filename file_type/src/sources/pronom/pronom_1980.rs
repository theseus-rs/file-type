use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1980: FileFormat = FileFormat {
    id: 1_980,
    source_type: SourceType::Pronom,
    name: "Alias Studio Wire File",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(12),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x41, 0x6C, 0x69, 0x61, 0x73, 0x20, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F,
                    ]),
                    Token::WildcardCount(13),
                    Token::Literal(&[0x38, 0x2E, 0x35]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_985,
    }],
};
