use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1743: FileFormat = FileFormat {
    id: 2_589,
    puid: "fmt/1743",
    name: "Nero Burning ROM Image File",
    extensions: &["nrg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::EOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4E, 0x45, 0x52]),
                    Token::Any(&[&[Token::Literal(&[0x35])], &[Token::Literal(&[0x4F])]]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_255,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
