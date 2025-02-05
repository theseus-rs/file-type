use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1945: FileFormat = FileFormat {
    id: 1_945,
    source_type: SourceType::Pronom,
    name: "SQLite Database File Format",
    extensions: &["sqlite", "db"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x2A, 0x2A, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        0x20, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x73, 0x20, 0x61, 0x6E,
                        0x20, 0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x32, 0x2E,
                    ]),
                    Token::SingleWildcard,
                    Token::Literal(&[
                        0x20, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x2A, 0x2A,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_528,
    }],
};
