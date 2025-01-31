use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_714: FileFormat = FileFormat {
    id: 1_513,
    puid: "fmt/714",
    name: "Extensible Music Format",
    extensions: &["xmf", "mxmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x4D, 0x46, 0x5F])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_766,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
