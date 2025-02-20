use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2610: FileType = FileType {
    file_format: &FileFormat {
        id: 2_610,
        source_type: SourceType::Pronom,
        name: "CloneCD Control File",
        extensions: &["ccd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x43, 0x6C, 0x6F, 0x6E, 0x65, 0x43, 0x44, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
