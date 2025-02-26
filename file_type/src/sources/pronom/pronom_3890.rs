use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3890: FileType = FileType {
    file_format: &FileFormat {
        id: 3_890,
        source_type: SourceType::Pronom,
        name: "Binvox",
        extensions: &["binvox"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x62, 0x69, 0x6E, 0x76, 0x6F, 0x78, 0x20, 0x31, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
