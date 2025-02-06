use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1933: FileFormat = FileFormat {
    id: 1_933,
    source_type: SourceType::Pronom,
    name: "Origin Project Format",
    extensions: &["opj", "ogg", "ogm", "ogw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x43, 0x50, 0x59, 0x41, 0x20]),
                    Token::Range(&[0x31], &[0x39]),
                    Token::Literal(&[0x2E]),
                    Token::WildcardCountRange(7, 13),
                    Token::Literal(&[0x23, 0x0A]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
