use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1777: FileFormat = FileFormat {
    id: 1_777,
    source_type: SourceType::Pronom,
    name: "Dolby MLP Lossless Audio",
    extensions: &["mlp"],
    media_types: &["audio/vnd.dolby.mlp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xF8, 0x72, 0x6F]),
                    Token::Any(&[&[Token::Literal(&[0xBA])], &[Token::Literal(&[0xBB])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
