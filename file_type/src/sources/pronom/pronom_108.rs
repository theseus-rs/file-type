use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_108: FileType = FileType {
    file_format: &FileFormat {
        id: 108,
        source_type: SourceType::Pronom,
        name: "OS/2 Presentation Manager Metafile (MET)",
        extensions: &["met"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD3, 0xA8, 0xA8])],
                },
            }],
        }],
        related_formats: &[],
    },
};
