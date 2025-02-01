use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1553: FileFormat = FileFormat {
    id: 2_378,
    puid: "fmt/1553",
    name: "Septentrio Binary Format",
    extensions: &["sbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x24, 0x40]),
                    Token::WildcardCountRange(460, 3_000),
                    Token::Literal(&[
                        0x50, 0x6F, 0x6C, 0x61, 0x52, 0x78, 0x53, 0x5F, 0x50, 0x52, 0x4F,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
