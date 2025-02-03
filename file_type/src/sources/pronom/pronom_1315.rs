use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1315: FileFormat = FileFormat {
    id: 1_315,
    source_type: SourceType::Pronom,
    name: "Multiple-image Network Graphics",
    extensions: &["mng"],
    media_types: &["video/x-mng"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x8A, 0x4D, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x1C,
                        0x4D, 0x48, 0x44, 0x52,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x45, 0x4E, 0x44, 0x21, 0x20, 0xF7, 0xD5,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
