use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2753: FileType = FileType {
    file_format: &FileFormat {
        id: 2_753,
        source_type: SourceType::Pronom,
        name: "Ptex File Format",
        extensions: &["ptx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x74, 0x65, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
