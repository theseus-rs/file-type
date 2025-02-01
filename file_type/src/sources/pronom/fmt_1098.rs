use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1098: FileFormat = FileFormat {
    id: 1_907,
    puid: "fmt/1098",
    name: "XZ File Format",
    extensions: &["xz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x5A])],
                },
            },
        ],
    }],
    related_formats: &[],
};
