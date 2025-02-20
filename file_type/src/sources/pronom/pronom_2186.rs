use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2186: FileType = FileType {
    file_format: &FileFormat {
        id: 2_186,
        source_type: SourceType::Pronom,
        name: "Nero CoverDesigner File",
        extensions: &["ncd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x56, 0x45, 0x52, 0x20, 0x45, 0x44, 0x49, 0x54, 0x4F, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
