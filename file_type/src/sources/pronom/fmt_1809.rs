use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1809: FileFormat = FileFormat {
    id: 2_660,
    puid: "fmt/1809",
    name: "Microsoft Access Encrypted Database File",
    extensions: &["mdb", "mda"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x77, 0x2C, 0x53, 0x20]),
                    Token::WildcardCount(1_030),
                    Token::Literal(&[0x68]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_659,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 107,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
