use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2749: FileType = FileType {
    file_format: &FileFormat {
        id: 2_749,
        source_type: SourceType::Pronom,
        name: "Microsoft Agent File",
        extensions: &["acs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC3, 0xAB, 0xCD, 0xAB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
