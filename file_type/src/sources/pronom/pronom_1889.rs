use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1889: FileType = FileType {
    file_format: &FileFormat {
        id: 1_889,
        source_type: SourceType::Pronom,
        name: "ASP Control Directive File",
        extensions: &["ascx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x25, 0x40, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C, 0x20,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[
                                0x41, 0x75, 0x74, 0x6F, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x57, 0x69,
                                0x72, 0x65, 0x75, 0x70,
                            ])],
                            &[Token::Literal(&[
                                0x43, 0x6C, 0x61, 0x73, 0x73, 0x4E, 0x61, 0x6D, 0x65,
                            ])],
                            &[Token::Literal(&[
                                0x43, 0x6F, 0x64, 0x65, 0x42, 0x65, 0x68, 0x69, 0x6E, 0x64,
                            ])],
                            &[Token::Literal(&[
                                0x43, 0x6F, 0x64, 0x65, 0x46, 0x69, 0x6C, 0x65,
                            ])],
                            &[Token::Literal(&[
                                0x43, 0x6F, 0x64, 0x65, 0x46, 0x69, 0x6C, 0x65, 0x42, 0x61, 0x73,
                                0x65, 0x43, 0x6C, 0x61, 0x73, 0x73,
                            ])],
                            &[Token::Literal(&[
                                0x43, 0x6F, 0x6D, 0x70, 0x69, 0x6C, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                                0x4D, 0x6F, 0x64, 0x65,
                            ])],
                            &[Token::Literal(&[
                                0x43, 0x6F, 0x6D, 0x70, 0x69, 0x6C, 0x65, 0x72, 0x4F, 0x70, 0x74,
                                0x69, 0x6F, 0x6E, 0x73,
                            ])],
                            &[Token::Literal(&[0x44, 0x65, 0x62, 0x75, 0x67])],
                            &[Token::Literal(&[
                                0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E,
                            ])],
                            &[Token::Literal(&[
                                0x45, 0x6E, 0x61, 0x62, 0x6C, 0x65, 0x54, 0x68, 0x65, 0x6D, 0x69,
                                0x6E, 0x67,
                            ])],
                            &[Token::Literal(&[
                                0x45, 0x6E, 0x61, 0x62, 0x6C, 0x65, 0x56, 0x69, 0x65, 0x77, 0x53,
                                0x74, 0x61, 0x74, 0x65,
                            ])],
                            &[Token::Literal(&[
                                0x45, 0x78, 0x70, 0x6C, 0x69, 0x63, 0x69, 0x74,
                            ])],
                            &[Token::Literal(&[
                                0x49, 0x6E, 0x68, 0x65, 0x72, 0x69, 0x74, 0x73,
                            ])],
                            &[Token::Literal(&[
                                0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65,
                            ])],
                            &[Token::Literal(&[
                                0x4C, 0x69, 0x6E, 0x65, 0x50, 0x72, 0x61, 0x67, 0x6D, 0x61, 0x73,
                            ])],
                            &[Token::Literal(&[0x53, 0x72, 0x63])],
                            &[Token::Literal(&[0x53, 0x74, 0x72, 0x69, 0x63, 0x74])],
                            &[Token::Literal(&[
                                0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x53, 0x63, 0x68, 0x65, 0x6D,
                                0x61,
                            ])],
                            &[Token::Literal(&[
                                0x57, 0x61, 0x72, 0x6E, 0x69, 0x6E, 0x67, 0x4C, 0x65, 0x76, 0x65,
                                0x6C,
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
