use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_182: FileType = FileType {
    file_format: &FileFormat {
        id: 182,
        source_type: SourceType::Pronom,
        name: "AutoCAD Xref Log",
        extensions: &["xlg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x0D, 0x0A, 0x0D, 0x0A, 0x0D, 0x0A, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D,
                            0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D, 0x3D,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x4F, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x3A, 0x20,
                        ]),
                        Token::Any(&[
                            &[Token::Literal(&[
                                0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x20, 0x58, 0x72, 0x65, 0x66,
                            ])],
                            &[Token::Literal(&[
                                0x4F, 0x76, 0x65, 0x72, 0x6C, 0x61, 0x79, 0x20, 0x58, 0x72, 0x65,
                                0x66,
                            ])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
