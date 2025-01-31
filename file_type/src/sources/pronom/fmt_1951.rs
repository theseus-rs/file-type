use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1951: FileFormat = FileFormat {
    id: 2_814,
    puid: "fmt/1951",
    name: "Pro Tools Session File",
    extensions: &["ptf", "pts"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x03, 0x30, 0x30, 0x31, 0x30, 0x31, 0x31, 0x31, 0x31, 0x30, 0x30, 0x31,
                        0x30, 0x31, 0x30, 0x31, 0x31,
                    ]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x01]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
