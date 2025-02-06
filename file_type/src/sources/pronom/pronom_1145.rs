use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1145: FileFormat = FileFormat {
    id: 1_145,
    source_type: SourceType::Pronom,
    name: "Enigma Binary File (Finale)",
    extensions: &["mus"],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_839,
    }],
};
