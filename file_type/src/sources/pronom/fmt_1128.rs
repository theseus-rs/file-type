use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1128: FileFormat = FileFormat {
    id: 1_938,
    puid: "fmt/1128",
    name: "Progressive Graphics File",
    extensions: &["pgf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x47, 0x46]),
                    Token::Any(&[
                        &[Token::Literal(&[0x02])],
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
