use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1692: FileType = FileType {
    file_format: &FileFormat {
        id: 1_692,
        source_type: SourceType::Pronom,
        name: "QuadriSpace Format",
        extensions: &["qsd", "qsl", "qsm", "qst"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x9F, 0x08, 0x7C, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
