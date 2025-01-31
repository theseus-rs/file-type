use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1727: FileFormat = FileFormat {
    id: 2_571,
    puid: "fmt/1727",
    name: "Pro Tools Session File",
    extensions: &["ptx"],
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
                    Token::WildcardCount(3),
                    Token::Literal(&[0x5A, 0x01, 0x00, 0x04]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
