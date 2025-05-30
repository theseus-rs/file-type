use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1664: FileType = FileType {
    file_format: &FileFormat {
        id: 1_664,
        source_type: SourceType::Pronom,
        name: "Maya Binary File Format",
        extensions: &["mb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x4F, 0x52, 0x38]),
                        Token::WildcardCount(12),
                        Token::Any(&[
                            &[Token::Literal(&[0x4D, 0x61, 0x79, 0x61])],
                            &[Token::Literal(&[0x4D, 0x41, 0x59, 0x41])],
                        ]),
                        Token::Literal(&[0x46, 0x4F, 0x52, 0x38]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
