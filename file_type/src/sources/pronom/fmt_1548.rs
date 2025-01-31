use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1548: FileFormat = FileFormat {
    id: 2_373,
    puid: "fmt/1548",
    name: "Visual Basics MAK File",
    extensions: &["mak"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::EOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x50, 0x72, 0x6F, 0x6A, 0x57, 0x69, 0x6E, 0x53, 0x69, 0x7A, 0x65, 0x3D,
                    ]),
                    Token::WildcardCountRange(7, 15),
                    Token::Literal(&[
                        0x0D, 0x0A, 0x50, 0x72, 0x6F, 0x6A, 0x57, 0x69, 0x6E, 0x53, 0x68, 0x6F,
                        0x77, 0x3D,
                    ]),
                    Token::WildcardCountRange(1, 3),
                    Token::Literal(&[
                        0x0D, 0x0A, 0x49, 0x63, 0x6F, 0x6E, 0x46, 0x6F, 0x72, 0x6D, 0x3D, 0x22,
                        0x46, 0x6F, 0x72, 0x6D,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
