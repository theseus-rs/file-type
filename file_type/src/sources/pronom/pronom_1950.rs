use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1950: FileType = FileType {
    file_format: &FileFormat {
        id: 1_950,
        source_type: SourceType::Pronom,
        name: "VectorWorks",
        extensions: &["vwx"],
        media_types: &["application/vnd.vectorworks"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(14),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x20,
                        0x31, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
