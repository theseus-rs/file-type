use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3928: FileType = FileType {
    file_format: &FileFormat {
        id: 3_928,
        source_type: SourceType::Pronom,
        name: "Apache Arrow IPC Format",
        extensions: &["arrow"],
        media_types: &["vnd.apache.arrow.file"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x41, 0x52, 0x52, 0x4F, 0x57, 0x31])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x41, 0x52, 0x52, 0x4F, 0x57, 0x31])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
