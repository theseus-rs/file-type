use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1135: FileFormat = FileFormat {
    id: 1_945,
    puid: "fmt/1135",
    name: "SQLite Database File Format",
    extensions: &["sqlite", "db"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        id: 1_528,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
