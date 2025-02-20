use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1777: FileType = FileType {
    file_format: &FileFormat {
        id: 1_777,
        source_type: SourceType::Pronom,
        name: "Dolby MLP Lossless Audio",
        extensions: &["mlp"],
        media_types: &["audio/vnd.dolby.mlp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xF8, 0x72, 0x6F]),
                        Token::Any(&[&[Token::Literal(&[0xBA])], &[Token::Literal(&[0xBB])]]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
