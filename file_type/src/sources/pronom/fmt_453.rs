use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_453: FileFormat = FileFormat {
    id: 1_240,
    puid: "fmt/453",
    name: "Verity Collection Stop List",
    extensions: &["stp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
