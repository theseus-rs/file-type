use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_364: FileFormat = FileFormat {
    id: 364,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Personal Folders (ANSI)",
    extensions: &["pst"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x21, 0x42, 0x44, 0x4E]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x53, 0x4D]),
                    Token::Any(&[
                        &[Token::Literal(&[0x0E, 0x00])],
                        &[Token::Literal(&[0x0F, 0x00])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
