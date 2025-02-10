use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2442: FileType = FileType {
    file_format: &FileFormat {
        id: 2_442,
        source_type: SourceType::Pronom,
        name: "CATIA Drawing",
        extensions: &["catdrawing"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x56, 0x35, 0x5F, 0x43, 0x46, 0x56, 0x32, 0x00]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x43, 0x41, 0x54, 0x44, 0x72, 0x77, 0x43, 0x6F, 0x6E, 0x74,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
