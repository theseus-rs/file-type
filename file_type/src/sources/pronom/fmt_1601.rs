use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1601: FileFormat = FileFormat {
    id: 2_428,
    puid: "fmt/1601",
    name: "Type Library",
    extensions: &["tlb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4C, 0x54, 0x47])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(13),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x59, 0x50, 0x45, 0x4C, 0x49, 0x42])],
                },
            },
        ],
    }],
    related_formats: &[],
};
