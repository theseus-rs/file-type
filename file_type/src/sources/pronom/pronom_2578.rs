use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2578: FileType = FileType {
    file_format: &FileFormat {
        id: 2_578,
        source_type: SourceType::Pronom,
        name: "PaintShop Plus Compressed Format",
        extensions: &["psc", "da4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x6D, 0x38, 0x39, 0x50, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
