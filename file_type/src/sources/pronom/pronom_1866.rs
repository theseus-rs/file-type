use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1866: FileFormat = FileFormat {
    id: 1_866,
    source_type: SourceType::Pronom,
    name: "Phase One Raw Image",
    extensions: &["cap", "capture"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x4D, 0x4D, 0x52, 0x61, 0x77, 0x54,
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
                        0x49, 0x49, 0x49, 0x49, 0x54, 0x77, 0x61, 0x52,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
