use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1606: FileFormat = FileFormat {
    id: 2_433,
    puid: "fmt/1606",
    name: "Packed-Ice True Colour Sprites [Spooky Sprites] (Atari Falcon)",
    extensions: &["trs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x49, 0x43, 0x45, 0x21]),
                    Token::WildcardCountRange(4, 11),
                    Token::Literal(&[0x54, 0x43, 0x53, 0x46]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_432,
        relationship_type: RelationshipType::IsSupertypeOf,
    }],
};
