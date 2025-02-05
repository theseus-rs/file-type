use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2433: FileFormat = FileFormat {
    id: 2_433,
    source_type: SourceType::Pronom,
    name: "Packed-Ice True Colour Sprites [Spooky Sprites] (Atari Falcon)",
    extensions: &["trs"],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::IsSupertypeOf,
        id: 2_432,
    }],
};
