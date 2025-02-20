use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2552: FileType = FileType {
    file_format: &FileFormat {
        id: 2_552,
        source_type: SourceType::Pronom,
        name: "Cintel Raw Image/DaVinci Resolve Image",
        extensions: &["cri", "dvcc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x44, 0x56, 0x43, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
