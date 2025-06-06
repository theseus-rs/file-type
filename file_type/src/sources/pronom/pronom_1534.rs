use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1534: FileType = FileType {
    file_format: &FileFormat {
        id: 1_534,
        source_type: SourceType::Pronom,
        name: "Dolby Digital AC-3",
        extensions: &["ac3"],
        media_types: &["audio/ac3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0B, 0x77])],
                },
            }],
        }],
        related_formats: &[],
    },
};
