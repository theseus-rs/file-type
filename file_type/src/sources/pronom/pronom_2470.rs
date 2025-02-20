use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2470: FileType = FileType {
    file_format: &FileFormat {
        id: 2_470,
        source_type: SourceType::Pronom,
        name: "Lenel Network Video Recorder File",
        extensions: &["lnr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4E, 0x52, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
