use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2480: FileType = FileType {
    file_format: &FileFormat {
        id: 2_480,
        source_type: SourceType::Pronom,
        name: "STAD PAC File",
        extensions: &["pac", "seq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x70, 0x4D, 0x38]),
                        Token::Any(&[&[Token::Literal(&[0x35])], &[Token::Literal(&[0x36])]]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
