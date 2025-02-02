use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2651: FileFormat = FileFormat {
    id: 2_651,
    source_type: SourceType::Pronom,
    name: "Multimedia Viewer Book",
    extensions: &["mvb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3F, 0x5F, 0x03, 0x00]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x6C, 0x03, 0x1B, 0x00, 0x01, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
