use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2687: FileType = FileType {
    file_format: &FileFormat {
        id: 2_687,
        source_type: SourceType::Pronom,
        name: "Archiver Format",
        extensions: &["a"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
