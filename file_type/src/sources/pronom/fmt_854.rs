use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_854: FileFormat = FileFormat {
    id: 1_655,
    puid: "fmt/854",
    name: "Personal Ancestral File (PAF)",
    extensions: &["paf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x46, 0x00, 0x33, 0x30, 0x30, 0x00, 0x33, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_656,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
