use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1415: FileFormat = FileFormat {
    id: 2_233,
    puid: "fmt/1415",
    name: "GST Publisher File",
    extensions: &["dtp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x54, 0x50, 0x49])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4F, 0x44, 0x46])],
                },
            },
        ],
    }],
    related_formats: &[],
};
