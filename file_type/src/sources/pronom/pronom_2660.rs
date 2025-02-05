use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2660: FileFormat = FileFormat {
    id: 2_660,
    source_type: SourceType::Pronom,
    name: "Microsoft Access Encrypted Database File",
    extensions: &["mdb", "mda"],
    media_types: &[],
    signatures: &[Signature {
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
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_659,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 107,
        },
    ],
};
