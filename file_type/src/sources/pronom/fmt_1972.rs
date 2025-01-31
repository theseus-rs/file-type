use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1972: FileFormat = FileFormat {
    id: 2_839,
    puid: "fmt/1972",
    name: "Enigma Binary File (Finale)",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x69, 0x6E, 0x61, 0x6C, 0x65, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_145,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_837,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
