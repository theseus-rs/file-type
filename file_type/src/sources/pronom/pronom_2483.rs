use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2483: FileType = FileType {
    file_format: &FileFormat {
        id: 2_483,
        source_type: SourceType::Pronom,
        name: "Microsoft Help Contents File",
        extensions: &["cnt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3A, 0x42, 0x61, 0x73, 0x65]),
                        Token::WildcardCountRange(1, 259),
                        Token::Literal(&[0x2E]),
                        Token::Any(&[
                            &[Token::Literal(&[0x48, 0x4C, 0x50])],
                            &[Token::Literal(&[0x68, 0x6C, 0x70])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
