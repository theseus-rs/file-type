use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2529: FileFormat = FileFormat {
    id: 2_529,
    source_type: SourceType::Pronom,
    name: "Asymetrix Compel Presentation",
    extensions: &["cpl", "art"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x03, 0x4A, 0x42, 0x4F, 0x4E, 0xD3, 0x2C, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_530,
    }],
};
