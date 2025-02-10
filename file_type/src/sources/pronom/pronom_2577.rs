use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2577: FileType = FileType {
    file_format: &FileFormat {
        id: 2_577,
        source_type: SourceType::Pronom,
        name: "Prism Paint Bitmap",
        extensions: &["pnt", "tpi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4E, 0x54, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
