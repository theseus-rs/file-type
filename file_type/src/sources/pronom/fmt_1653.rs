use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1653: FileFormat = FileFormat {
    id: 2_480,
    puid: "fmt/1653",
    name: "STAD PAC File",
    extensions: &["pac", "seq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x70, 0x4D, 0x38]),
                    Token::Any(&[&[Token::Literal(&[0x35])], &[Token::Literal(&[0x36])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
