use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1361: FileFormat = FileFormat {
    id: 2_179,
    puid: "fmt/1361",
    name: "Amiga Disk File",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x4F, 0x53]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00])],
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x03])],
                        &[Token::Literal(&[0x04])],
                        &[Token::Literal(&[0x05])],
                        &[Token::Literal(&[0x06])],
                        &[Token::Literal(&[0x07])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
