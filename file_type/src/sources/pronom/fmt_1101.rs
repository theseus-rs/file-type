use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1101: FileFormat = FileFormat {
    id: 1_909,
    puid: "fmt/1101",
    name: "High Efficiency Image File Format",
    extensions: &["heic"],
    media_types: &["image/heif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x66, 0x74, 0x79, 0x70]),
                    Token::Any(&[
                        &[Token::Literal(&[0x6D, 0x69, 0x66, 0x31])],
                        &[Token::Literal(&[0x6D, 0x73, 0x66, 0x31])],
                        &[Token::Literal(&[0x68, 0x65, 0x69, 0x63])],
                        &[Token::Literal(&[0x68, 0x65, 0x69, 0x78])],
                        &[Token::Literal(&[0x68, 0x65, 0x76, 0x63])],
                        &[Token::Literal(&[0x68, 0x65, 0x76, 0x78])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
