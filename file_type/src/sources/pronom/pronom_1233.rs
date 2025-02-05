use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1233: FileFormat = FileFormat {
    id: 1_233,
    source_type: SourceType::Pronom,
    name: "Adobe Portable Document Catalog Index File",
    extensions: &["pdx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x50, 0x44, 0x58, 0x20, 0x32, 0x2E, 0x30, 0x0D, 0x0A, 0x0D, 0x0A,
                        0x4F, 0x62, 0x6A, 0x42, 0x65, 0x67, 0x69, 0x6E, 0x20, 0x0D, 0x0A, 0x3C,
                        0x3C,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3E, 0x3E, 0x20, 0x0D, 0x0A, 0x4F, 0x62, 0x6A, 0x45, 0x6E, 0x64, 0x20,
                        0x0D, 0x0A, 0x0D, 0x0A,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
