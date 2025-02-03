use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1361: FileFormat = FileFormat {
    id: 1_361,
    source_type: SourceType::Pronom,
    name: "WebM",
    extensions: &["webm"],
    media_types: &["video/webm"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x1A, 0x45, 0xDF, 0xA3]),
                    Token::WildcardCountRange(0, 32),
                    Token::Literal(&[0x42, 0x82, 0x84, 0x77, 0x65, 0x62, 0x6D, 0x42, 0x87]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
