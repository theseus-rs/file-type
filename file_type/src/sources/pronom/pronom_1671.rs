use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1671: FileType = FileType {
    file_format: &FileFormat {
        id: 1_671,
        source_type: SourceType::Pronom,
        name: "Microsoft Reader eBook",
        extensions: &["lit"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x54, 0x4F, 0x4C, 0x49, 0x54, 0x4C, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
