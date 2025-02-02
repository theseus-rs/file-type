use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2633: FileFormat = FileFormat {
    id: 2_633,
    source_type: SourceType::Pronom,
    name: "The Spectral Geologist Dataset",
    extensions: &["tsg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x53, 0x47, 0x20, 0x37, 0x2E, 0x30, 0x30, 0x20, 0x44, 0x61, 0x74, 0x61,
                    0x73, 0x65, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_632,
    }],
};
