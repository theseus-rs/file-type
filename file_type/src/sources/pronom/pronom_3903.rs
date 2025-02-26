use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3903: FileType = FileType {
    file_format: &FileFormat {
        id: 3_903,
        source_type: SourceType::Pronom,
        name: "Apache Avro",
        extensions: &["avro"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4F, 0x62, 0x6A, 0x01]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x61, 0x76, 0x72, 0x6F, 0x2E]),
                        Token::Any(&[
                            &[Token::Literal(&[0x63, 0x6F, 0x64, 0x65, 0x63])],
                            &[Token::Literal(&[0x73, 0x79, 0x6E, 0x63])],
                        ]),
                        Token::WildcardCountRange(8, 50),
                        Token::Literal(&[0x73, 0x63, 0x68, 0x65, 0x6D, 0x61]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x22, 0x74, 0x79, 0x70, 0x65, 0x22]),
                        Token::WildcardCountRange(2, 65),
                        Token::Literal(&[0x22, 0x6E, 0x61, 0x6D, 0x65, 0x22]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
