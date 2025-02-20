use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2261: FileType = FileType {
    file_format: &FileFormat {
        id: 2_261,
        source_type: SourceType::Pronom,
        name: "QuarkXPress Document",
        extensions: &[],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x36, 0x00, 0x36])],
                },
            }],
        }],
        related_formats: &[],
    },
};
