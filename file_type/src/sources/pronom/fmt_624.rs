use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_624: FileFormat = FileFormat {
    id: 1_420,
    puid: "fmt/624",
    name: "RIFF Palette Format",
    extensions: &["pal"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x50, 0x41, 0x4C, 0x20, 0x64, 0x61, 0x74, 0x61]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_741,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
