use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1605: FileFormat = FileFormat {
    id: 2_432,
    puid: "fmt/1605",
    name: "True Colour Sprites [Spooky Sprites] (Atari Falcon)",
    extensions: &["trs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x43, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_433,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
