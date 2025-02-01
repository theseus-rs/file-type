use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_53: FileFormat = FileFormat {
    id: 87,
    puid: "x-fmt/53",
    name: "Macromedia Freehand",
    extensions: &["fh5", "fh4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x47, 0x44, 0x31])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 458,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
