use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2822: FileType = FileType {
    file_format: &FileFormat {
        id: 2_822,
        source_type: SourceType::Pronom,
        name: "Program Embroidery Stitch (PES) File",
        extensions: &["pes"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x50, 0x45, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
