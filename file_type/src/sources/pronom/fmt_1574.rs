use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1574: FileFormat = FileFormat {
    id: 2_399,
    puid: "fmt/1574",
    name: "Visual Basic Project Workspace File",
    extensions: &["vbw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x72, 0x6D])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 2_366,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
