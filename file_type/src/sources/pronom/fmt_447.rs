use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_447: FileFormat = FileFormat {
    id: 1_234,
    puid: "fmt/447",
    name: "Adobe Portable Document Catalog Index File",
    extensions: &["pdx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x50, 0x44, 0x58, 0x2D, 0x33, 0x2E, 0x30, 0x0D, 0x25, 0xE2, 0xE3,
                        0xCF, 0xD3, 0x0D, 0x0A, 0x31, 0x20, 0x30, 0x20, 0x6F, 0x62, 0x6A, 0x3C,
                        0x3C, 0x2F, 0x50, 0x44, 0x58, 0x3C, 0x3C,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3E, 0x3E, 0x0D, 0x0A, 0x25, 0x25, 0x45, 0x4F, 0x46, 0x0D, 0x0A,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
