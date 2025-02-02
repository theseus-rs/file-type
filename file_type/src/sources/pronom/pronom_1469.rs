use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1469: FileFormat = FileFormat {
    id: 1_469,
    source_type: SourceType::Pronom,
    name: "PKCS #7 Cryptographic Message File",
    extensions: &["p7m", "p7b", "p7s"],
    media_types: &["application/pkcs7-mime", "application/pkcs7-signature"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x30]),
                    Token::WildcardCountRange(1, 5),
                    Token::Literal(&[0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_061,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_062,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_063,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        },
    ],
};
