use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2009: FileFormat = FileFormat {
    id: 2_009,
    source_type: SourceType::Pronom,
    name: "RData",
    extensions: &["rdata"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x44, 0x41, 0x32, 0x0A, 0x41, 0x0A])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::CanBeContainedBy,
        id: 386,
    }],
};
