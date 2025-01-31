use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_959: FileFormat = FileFormat {
    id: 1_764,
    puid: "fmt/959",
    name: "Portable Sound Format",
    extensions: &[
        "psf", "psf1", "psflib", "minipsf", "minipsf1", "gsf", "gsflib", "minigsf",
    ],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x53, 0x46]),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x11])],
                        &[Token::Literal(&[0x12])],
                        &[Token::Literal(&[0x13])],
                        &[Token::Literal(&[0x21])],
                        &[Token::Literal(&[0x22])],
                        &[Token::Literal(&[0x23])],
                        &[Token::Literal(&[0x41])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
