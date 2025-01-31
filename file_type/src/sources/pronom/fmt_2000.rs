use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_2000: FileFormat = FileFormat {
    id: 2_874,
    puid: "fmt/2000",
    name: "Husqvarna Embroidery Stitch File",
    extensions: &["hus"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Any(&[&[Token::Literal(&[0x5B])], &[Token::Literal(&[0x5D])]]),
                    Token::Any(&[
                        &[Token::Literal(&[0xAF])],
                        &[Token::Literal(&[0xFF])],
                        &[Token::Literal(&[0xFC])],
                    ]),
                    Token::Literal(&[0xC8, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_876,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
