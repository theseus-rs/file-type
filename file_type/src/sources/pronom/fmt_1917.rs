use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1917: FileFormat = FileFormat {
    id: 2_775,
    puid: "fmt/1917",
    name: "BigTIFF",
    extensions: &["tif", "tf8", "btf"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x2B, 0x00, 0x08, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x00, 0x2B, 0x00, 0x08, 0x00, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 1_702,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
