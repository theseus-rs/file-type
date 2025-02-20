use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2700: FileType = FileType {
    file_format: &FileFormat {
        id: 2_700,
        source_type: SourceType::Pronom,
        name: "Trelby Document File",
        extensions: &["trelby"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x23, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
