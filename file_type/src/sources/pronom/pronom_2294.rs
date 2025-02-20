use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2294: FileType = FileType {
    file_format: &FileFormat {
        id: 2_294,
        source_type: SourceType::Pronom,
        name: "Multi Palette Picture File",
        extensions: &["mpp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
