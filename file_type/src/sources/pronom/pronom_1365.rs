use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1365: FileFormat = FileFormat {
    id: 1_365,
    source_type: SourceType::Pronom,
    name: "Image Cytometry Standard",
    extensions: &["ics"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x09, 0x0A, 0x69, 0x63, 0x73, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x09, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_366,
    }],
};
