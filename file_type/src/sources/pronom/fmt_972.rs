use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_972: FileFormat = FileFormat {
    id: 1_777,
    puid: "fmt/972",
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
