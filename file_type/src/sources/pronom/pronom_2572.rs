use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2572: FileFormat = FileFormat {
    id: 2_572,
    source_type: SourceType::Pronom,
    name: "dBASE Windows Form File",
    extensions: &["wfm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x2A, 0x2A, 0x20, 0x45, 0x4E, 0x44, 0x20, 0x48, 0x45, 0x41, 0x44, 0x45,
                        0x52, 0x20, 0x2D, 0x2D, 0x20, 0x64, 0x6F, 0x20, 0x6E, 0x6F, 0x74, 0x20,
                        0x72, 0x65, 0x6D, 0x6F, 0x76, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
                        0x6C, 0x69, 0x6E, 0x65, 0x2A, 0x0D, 0x0A, 0x2A, 0x20, 0x47, 0x65, 0x6E,
                        0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6F, 0x6E, 0x20,
                    ]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x2F]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x2F]),
                    Token::WildcardCountRange(2, 4),
                    Token::Literal(&[0x0D, 0x0A, 0x2A]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
