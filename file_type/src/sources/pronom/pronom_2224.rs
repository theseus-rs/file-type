use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2224: FileType = FileType {
    file_format: &FileFormat {
        id: 2_224,
        source_type: SourceType::Pronom,
        name: "Flow Charting",
        extensions: &["cht"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Any(&[&[Token::Literal(&[0x78])], &[Token::Literal(&[0xC8])]]),
                        Token::Literal(&[0x00, 0x78, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
