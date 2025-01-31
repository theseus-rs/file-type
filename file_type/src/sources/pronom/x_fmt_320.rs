use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_320: FileFormat = FileFormat {
    id: 482,
    puid: "x-fmt/320",
    name: "Fractal Image",
    extensions: &["fif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x49, 0x46, 0x01]),
                    Token::WildcardCount(52),
                    Token::Literal(&[0xC0]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
