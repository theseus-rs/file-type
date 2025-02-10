use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2182: FileType = FileType {
    file_format: &FileFormat {
        id: 2_182,
        source_type: SourceType::Pronom,
        name: "V-Ray Material",
        extensions: &["vismat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x76, 0x69, 0x73, 0x6D, 0x61, 0x74, 0x3E]),
                        Token::WildcardCountRange(0, 128),
                        Token::Literal(&[
                            0x72, 0x65, 0x6E, 0x64, 0x65, 0x72, 0x65, 0x72, 0x3D, 0x22, 0x76, 0x72,
                            0x61, 0x79, 0x22, 0x3E,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
