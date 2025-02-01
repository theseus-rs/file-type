use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1204: FileFormat = FileFormat {
    id: 2_014,
    puid: "fmt/1204",
    name: "Strata StudioPro Vis Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x56, 0x69, 0x73, 0x33]),
                    Token::WildcardCountRange(7, 64),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x54, 0x41, 0x54, 0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x45,
                        ])],
                        &[Token::Literal(&[0x54, 0x47, 0x52, 0x4F, 0x55, 0x50])],
                        &[Token::Literal(&[0x54, 0x4D, 0x4F, 0x44, 0x45, 0x4C])],
                        &[Token::Literal(&[
                            0x54, 0x50, 0x4F, 0x4C, 0x59, 0x47, 0x4F, 0x4E,
                        ])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
