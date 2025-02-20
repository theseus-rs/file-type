use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_59: FileType = FileType {
    file_format: &FileFormat {
        id: 59,
        source_type: SourceType::Pronom,
        name: "CALS Compressed Bitmap",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x72, 0x63, 0x64, 0x6F, 0x63, 0x69, 0x64, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
