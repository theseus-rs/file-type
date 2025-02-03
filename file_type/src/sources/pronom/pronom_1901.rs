use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1901: FileFormat = FileFormat {
    id: 1_901,
    source_type: SourceType::Pronom,
    name: "Alias Scene Description Language",
    extensions: &["sdl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x45, 0x46, 0x49, 0x4E, 0x49, 0x54, 0x49, 0x4F, 0x4E]),
                    Token::WildcardCountRange(1, 512),
                    Token::Literal(&[0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x4D, 0x4F, 0x44, 0x45, 0x4C]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
