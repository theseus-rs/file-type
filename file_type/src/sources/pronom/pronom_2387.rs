use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2387: FileType = FileType {
    file_format: &FileFormat {
        id: 2_387,
        source_type: SourceType::Pronom,
        name: "AutoDesk Indexed Point Cloud",
        extensions: &["pcg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
