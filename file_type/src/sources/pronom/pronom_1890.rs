use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1890: FileType = FileType {
    file_format: &FileFormat {
        id: 1_890,
        source_type: SourceType::Pronom,
        name: "ASP WebService Directive File",
        extensions: &["asmx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x25, 0x40, 0x20, 0x57, 0x65, 0x62, 0x53, 0x65, 0x72, 0x76, 0x69,
                            0x63, 0x65, 0x20,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[0x43, 0x6C, 0x61, 0x73, 0x73])],
                            &[Token::Literal(&[
                                0x43, 0x6F, 0x64, 0x65, 0x42, 0x65, 0x68, 0x69, 0x6E, 0x64,
                            ])],
                            &[Token::Literal(&[0x44, 0x65, 0x62, 0x75, 0x67])],
                            &[Token::Literal(&[
                                0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65,
                            ])],
                        ]),
                        Token::Literal(&[0x3D]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
