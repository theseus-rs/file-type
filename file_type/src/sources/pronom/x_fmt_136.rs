use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_136: FileFormat = FileFormat {
    id: 195,
    puid: "x-fmt/136",
    name: "Audio Interchange File Format (compressed)",
    extensions: &["aifc"],
    media_types: &["audio/x-aiff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x41, 0x49, 0x46, 0x43]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 221,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
