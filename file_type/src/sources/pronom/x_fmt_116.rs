use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_116: FileFormat = FileFormat {
    id: 168,
    puid: "x-fmt/116",
    name: "Lotus 1-2-3 Worksheet",
    extensions: &["wk4"],
    media_types: &["application/lotus123", "application/vnd.lotus-1-2-3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x1A, 0x00, 0x02, 0x10, 0x04, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 166,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 167,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 169,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
