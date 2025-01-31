use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1060: FileFormat = FileFormat {
    id: 1_866,
    puid: "fmt/1060",
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
