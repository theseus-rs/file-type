use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2435: FileFormat = FileFormat {
    id: 2_435,
    source_type: SourceType::Pronom,
    name: "Packed-Ice True Colour Picture [Spooky Sprites] (Atari Falcon)",
    extensions: &["trp", "tru"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x49, 0x43, 0x45, 0x21]),
                    Token::WildcardCountRange(4, 11),
                    Token::Literal(&[0x74, 0x72, 0x75, 0x3F]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSupertypeOf,
        id: 2_434,
    }],
};
