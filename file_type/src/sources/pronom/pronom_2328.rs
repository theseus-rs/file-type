use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2328: FileType = FileType {
    file_format: &FileFormat {
        id: 2_328,
        source_type: SourceType::Pronom,
        name: "Agisoft Point Cloud",
        extensions: &["oc3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x43, 0x01, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
