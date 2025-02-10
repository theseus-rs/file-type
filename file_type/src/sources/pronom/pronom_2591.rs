use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2591: FileType = FileType {
    file_format: &FileFormat {
        id: 2_591,
        source_type: SourceType::Pronom,
        name: "PixArt Bitmap",
        extensions: &["pix"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x58, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
