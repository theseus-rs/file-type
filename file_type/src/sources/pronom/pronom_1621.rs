use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1621: FileType = FileType {
    file_format: &FileFormat {
        id: 1_621,
        source_type: SourceType::Pronom,
        name: "G64 GCR-encoded Disk Image Format",
        extensions: &["g41", "g64", "g71"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x43, 0x52, 0x2D, 0x31, 0x35, 0x34, 0x31, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
