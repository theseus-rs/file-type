use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2877: FileType = FileType {
    file_format: &FileFormat {
        id: 2_877,
        source_type: SourceType::Pronom,
        name: "Husqvarna / TruE Embroidery Stitch File",
        extensions: &["vp3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x76, 0x73, 0x6D, 0x25, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
