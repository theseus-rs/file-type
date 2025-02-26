use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3913: FileType = FileType {
    file_format: &FileFormat {
        id: 3_913,
        source_type: SourceType::Pronom,
        name: "HxC Floppy Emulator Disk Image",
        extensions: &["hfe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x58, 0x43, 0x48, 0x46, 0x45, 0x56, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
