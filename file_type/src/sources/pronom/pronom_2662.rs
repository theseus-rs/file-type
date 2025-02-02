use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2662: FileFormat = FileFormat {
    id: 2_662,
    source_type: SourceType::Pronom,
    name: "Vips Image",
    extensions: &["v", "vips"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x08, 0xF2, 0xA6, 0xB6])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB6, 0xA6, 0xF2, 0x08])],
                },
            }],
        },
    ],
    related_formats: &[],
};
