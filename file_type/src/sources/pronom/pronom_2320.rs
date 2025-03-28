use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2320: FileType = FileType {
    file_format: &FileFormat {
        id: 2_320,
        source_type: SourceType::Pronom,
        name: "XV Thumbnail",
        extensions: &["p7"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x50, 0x37, 0x20, 0x33, 0x33, 0x32]),
                        Token::WildcardCount(1),
                        Token::Literal(&[
                            0x23, 0x58, 0x56, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
