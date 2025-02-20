use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2728: FileType = FileType {
    file_format: &FileFormat {
        id: 2_728,
        source_type: SourceType::Pronom,
        name: "Esko ArtPro File",
        extensions: &["ap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x72, 0x74, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
