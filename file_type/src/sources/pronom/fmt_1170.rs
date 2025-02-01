use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1170: FileFormat = FileFormat {
    id: 1_980,
    puid: "fmt/1170",
    name: "Alias Studio Wire File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        id: 1_985,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
