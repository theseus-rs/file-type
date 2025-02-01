use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1087: FileFormat = FileFormat {
    id: 1_895,
    puid: "fmt/1087",
    name: "FAT Disk Image",
    extensions: &["img", "ima", "dsk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xEB]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x90]),
                    Token::WildcardCount(10),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x04])],
                        &[Token::Literal(&[0x08])],
                        &[Token::Literal(&[0x10])],
                        &[Token::Literal(&[0x20])],
                        &[Token::Literal(&[0x40])],
                        &[Token::Literal(&[0x80])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_436,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
