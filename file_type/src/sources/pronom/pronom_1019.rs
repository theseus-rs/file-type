use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1019: FileFormat = FileFormat {
    id: 1_019,
    source_type: SourceType::Pronom,
    name: "FLAC (Free Lossless Audio Codec)",
    extensions: &["flac"],
    media_types: &["audio/flac"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x66, 0x4C, 0x61, 0x43]),
                    Token::Any(&[&[Token::Literal(&[0x80])], &[Token::Literal(&[0x00])]]),
                    Token::Literal(&[0x00, 0x00, 0x22]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
