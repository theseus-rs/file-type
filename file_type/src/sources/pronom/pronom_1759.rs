use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1759: FileType = FileType {
    file_format: &FileFormat {
        id: 1_759,
        source_type: SourceType::Pronom,
        name: "Adaptive Multi-Rate Wideband Audio",
        extensions: &["awb"],
        media_types: &["audio/amr-wb"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x21, 0x41, 0x4D, 0x52, 0x2D, 0x57, 0x42, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
