use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2814: FileType = FileType {
    file_format: &FileFormat {
        id: 2_814,
        source_type: SourceType::Pronom,
        name: "Pro Tools Session File",
        extensions: &["ptf", "pts"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x03, 0x30, 0x30, 0x31, 0x30, 0x31, 0x31, 0x31, 0x31, 0x30, 0x30, 0x31,
                            0x30, 0x31, 0x30, 0x31, 0x31,
                        ]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
