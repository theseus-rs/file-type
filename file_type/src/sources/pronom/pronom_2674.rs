use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2674: FileType = FileType {
    file_format: &FileFormat {
        id: 2_674,
        source_type: SourceType::Pronom,
        name: "Audacity Project File",
        extensions: &["aup"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x41, 0x75, 0x64, 0x61, 0x63, 0x69, 0x74, 0x79, 0x50, 0x72, 0x6F, 0x6A,
                            0x65, 0x63, 0x74,
                        ]),
                        Token::Any(&[&[Token::Literal(&[0x0D])], &[Token::Literal(&[0x0D, 0x0A])]]),
                        Token::Literal(&[0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E]),
                        Token::Any(&[&[Token::Literal(&[0x0D])], &[Token::Literal(&[0x0D, 0x0A])]]),
                        Token::WildcardCount(4),
                        Token::Any(&[&[Token::Literal(&[0x0D])], &[Token::Literal(&[0x0D, 0x0A])]]),
                        Token::Literal(&[0x70, 0x72, 0x6F, 0x6A, 0x4E, 0x61, 0x6D, 0x65]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
