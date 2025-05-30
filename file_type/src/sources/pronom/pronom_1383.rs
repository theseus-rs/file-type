use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1383: FileType = FileType {
    file_format: &FileFormat {
        id: 1_383,
        source_type: SourceType::Pronom,
        name: "Radiance RGBE Image Format",
        extensions: &["hdr", "pic", "rgbe", "xyze"],
        media_types: &["image/vnd.radiance"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x3F, 0x52, 0x41, 0x44, 0x49, 0x41, 0x4E, 0x43, 0x45, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
