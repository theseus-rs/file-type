use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_197: FileFormat = FileFormat {
    id: 197,
    source_type: SourceType::Pronom,
    name: "Electronic Arts Music",
    extensions: &["asf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x53, 0x43, 0x48, 0x6C]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x50, 0x54]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
