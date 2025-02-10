use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1433: FileType = FileType {
    file_format: &FileFormat {
        id: 1_433,
        source_type: SourceType::Pronom,
        name: "Microsoft Compiled HTML Help",
        extensions: &["chm", "chw"],
        media_types: &["application/vnd.ms-htmlhelp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x54, 0x53, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
