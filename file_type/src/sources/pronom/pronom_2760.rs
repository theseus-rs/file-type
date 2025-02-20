use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2760: FileType = FileType {
    file_format: &FileFormat {
        id: 2_760,
        source_type: SourceType::Pronom,
        name: "Pasti Floppy Disk Image",
        extensions: &["stx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x59])],
                },
            }],
        }],
        related_formats: &[],
    },
};
