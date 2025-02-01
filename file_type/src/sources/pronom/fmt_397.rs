use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_397: FileFormat = FileFormat {
    id: 1_145,
    puid: "fmt/397",
    name: "Enigma Binary File (Finale)",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x4E, 0x49, 0x47, 0x4D, 0x41, 0x20, 0x42, 0x49, 0x4E, 0x41, 0x52, 0x59,
                    0x20, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_839,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
