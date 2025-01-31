use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1607: FileFormat = FileFormat {
    id: 2_434,
    puid: "fmt/1607",
    name: "True Colour Picture [Spooky Sprites] (Atari Falcon)",
    extensions: &["trp", "tru"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x72, 0x75, 0x3F])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_435,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
