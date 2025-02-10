use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2526: FileType = FileType {
    file_format: &FileFormat {
        id: 2_526,
        source_type: SourceType::Pronom,
        name: "Microsoft Word for MS-DOS Style Sheet File",
        extensions: &["sty"],
        media_types: &["application/msword"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x31, 0xBE, 0x02, 0x00, 0x00, 0xAB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
