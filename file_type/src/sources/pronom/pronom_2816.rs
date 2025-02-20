use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2816: FileType = FileType {
    file_format: &FileFormat {
        id: 2_816,
        source_type: SourceType::Pronom,
        name: "PechaMaker Format",
        extensions: &["pxp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x70, 0x78, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
