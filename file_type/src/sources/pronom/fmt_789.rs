use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_789: FileFormat = FileFormat {
    id: 1_588,
    puid: "fmt/789",
    name: "Material Exchange Format",
    extensions: &["mxf"],
    media_types: &["application/mxf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x06, 0x0E, 0x2B, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0D, 0x01, 0x02, 0x01,
                        0x01, 0x02,
                    ]),
                    Token::WildcardCount(70),
                    Token::Literal(&[
                        0x06, 0x0E, 0x2B, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0D, 0x01, 0x02, 0x01,
                        0x03, 0x02,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
