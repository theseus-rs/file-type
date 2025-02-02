use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1075: FileFormat = FileFormat {
    id: 1_075,
    source_type: SourceType::Pronom,
    name: "Peak Graphical Waveform File",
    extensions: &["pk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xF1, 0x06, 0x00, 0x00, 0x00]),
                    Token::Range(&[0x01], &[0x02]),
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
