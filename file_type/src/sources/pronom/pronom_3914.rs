use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3914: FileType = FileType {
    file_format: &FileFormat {
        id: 3_914,
        source_type: SourceType::Pronom,
        name: "HxC Floppy Emulator Stream Image",
        extensions: &["hfe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x78, 0x43, 0x5F, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x5F, 0x49,
                        0x6D, 0x61, 0x67, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
