use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_761: FileFormat = FileFormat {
    id: 761,
    source_type: SourceType::Pronom,
    name: "Drawing Interchange Binary Format",
    extensions: &["dxb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x44, 0x58, 0x42, 0x20,
                        0x31, 0x2E, 0x30, 0x0D, 0x0A, 0x1A, 0x00,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00])],
                },
            },
        ],
    }],
    related_formats: &[],
};
