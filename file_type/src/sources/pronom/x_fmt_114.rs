use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_114: FileFormat = FileFormat {
    id: 166,
    puid: "x-fmt/114",
    name: "Lotus 1-2-3 Worksheet",
    extensions: &["wk1", "wk2"],
    media_types: &["application/vnd.lotus-1-2-3", "application/x-123"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x02, 0x00, 0x06, 0x04, 0x06, 0x00, 0x08, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 167,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 168,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 169,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
