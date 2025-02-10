use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1814: FileType = FileType {
    file_format: &FileFormat {
        id: 1_814,
        source_type: SourceType::Pronom,
        name: "FBX (Filmbox) Binary",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x61, 0x79, 0x64, 0x61, 0x72, 0x61, 0x20, 0x46, 0x42, 0x58, 0x20,
                        0x42, 0x69, 0x6E, 0x61, 0x72, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
