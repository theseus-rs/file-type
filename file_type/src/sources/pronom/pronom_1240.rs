use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1240: FileFormat = FileFormat {
    id: 1_240,
    source_type: SourceType::Pronom,
    name: "Verity Collection Stop List",
    extensions: &["stp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x63, 0x64, 0x65, 0x6B, 0x6B, 0x6D, 0x73, 0x0D, 0x0A,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A])],
                },
            },
        ],
    }],
    related_formats: &[],
};
