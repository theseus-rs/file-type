use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2017: FileType = FileType {
    file_format: &FileFormat {
        id: 2_017,
        source_type: SourceType::Pronom,
        name: "Sony SFK File",
        extensions: &["sfk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x50, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
