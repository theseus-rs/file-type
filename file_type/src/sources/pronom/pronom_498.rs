use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_498: FileFormat = FileFormat {
    id: 498,
    source_type: SourceType::Pronom,
    name: "Lotus Freelance Smartmaster Graphics",
    extensions: &["mas"],
    media_types: &["application/vnd.lotus-freelance"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x01, 0x40, 0x00, 0x44, 0x45, 0x42, 0x52, 0x40]),
                    Token::WildcardCount(155),
                    Token::Literal(&[
                        0x42, 0x75, 0x6C, 0x6C, 0x65, 0x74, 0x65, 0x64, 0x20, 0x4C, 0x69, 0x73,
                        0x74,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
