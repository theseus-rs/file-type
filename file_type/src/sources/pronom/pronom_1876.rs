use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1876: FileFormat = FileFormat {
    id: 1_876,
    source_type: SourceType::Pronom,
    name: "Cue Sheet",
    extensions: &["cue"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x49, 0x4C, 0x45]),
                    Token::WildcardCountRange(0, 1_024),
                    Token::Literal(&[0x20, 0x20, 0x54, 0x52, 0x41, 0x43, 0x4B, 0x20, 0x30, 0x31]),
                    Token::WildcardCountRange(0, 1_024),
                    Token::Literal(&[0x49, 0x4E, 0x44, 0x45, 0x58, 0x20, 0x30, 0x31, 0x20]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x35]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::Literal(&[0x3A]),
                    Token::Range(&[0x30], &[0x37]),
                    Token::Range(&[0x30], &[0x39]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
