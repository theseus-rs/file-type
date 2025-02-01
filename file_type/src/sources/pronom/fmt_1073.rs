use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1073: FileFormat = FileFormat {
    id: 1_880,
    puid: "fmt/1073",
    name: "Google Document Link File",
    extensions: &[
        "gslides", "gdoc", "gsheet", "gdraw", "gmap", "gsite", "gform",
    ],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x7B, 0x22, 0x75, 0x72, 0x6C, 0x22, 0x3A, 0x20, 0x22, 0x68, 0x74, 0x74,
                        0x70, 0x73, 0x3A, 0x2F, 0x2F, 0x64, 0x6F, 0x63, 0x73, 0x2E, 0x67, 0x6F,
                        0x6F, 0x67, 0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x2F, 0x6F, 0x70, 0x65,
                        0x6E,
                    ]),
                    Token::WildcardCountRange(1, 64),
                    Token::Literal(&[0x64, 0x6F, 0x63, 0x5F, 0x69, 0x64]),
                    Token::WildcardCountRange(1, 128),
                    Token::Literal(&[0x65, 0x6D, 0x61, 0x69, 0x6C]),
                    Token::WildcardCountRange(1, 128),
                    Token::Literal(&[
                        0x72, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x5F, 0x69, 0x64,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
