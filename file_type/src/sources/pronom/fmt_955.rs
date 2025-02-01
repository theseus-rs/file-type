use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_955: FileFormat = FileFormat {
    id: 1_760,
    puid: "fmt/955",
    name: "Downloadable Sounds Audio",
    extensions: &["dls"],
    media_types: &["audio/dls"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x44, 0x4C, 0x53, 0x20]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_741,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
