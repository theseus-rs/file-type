use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_424: FileType = FileType {
    file_format: &FileFormat {
        id: 424,
        source_type: SourceType::Pronom,
        name: "RealAudio",
        extensions: &["ra"],
        media_types: &["audio/vnd.rn-realaudio"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x72, 0x61, 0xFD, 0x00, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
