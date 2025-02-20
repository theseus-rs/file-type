use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2323: FileType = FileType {
    file_format: &FileFormat {
        id: 2_323,
        source_type: SourceType::Pronom,
        name: "Adobe Acrobat Forms Data Format",
        extensions: &["fdf"],
        media_types: &["application/vnd.fdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x46, 0x44, 0x46, 0x2D, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
