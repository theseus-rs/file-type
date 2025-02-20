use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2507: FileType = FileType {
    file_format: &FileFormat {
        id: 2_507,
        source_type: SourceType::Pronom,
        name: "Z Compressed Data",
        extensions: &["z"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x9D, 0x90])],
                },
            }],
        }],
        related_formats: &[],
    },
};
