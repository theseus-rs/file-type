use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2109: FileType = FileType {
    file_format: &FileFormat {
        id: 2_109,
        source_type: SourceType::Pronom,
        name: "RFFlow Chart",
        extensions: &["flo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x46, 0x4C, 0x4F, 0x37])],
                },
            }],
        }],
        related_formats: &[],
    },
};
