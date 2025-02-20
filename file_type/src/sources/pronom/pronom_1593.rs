use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1593: FileType = FileType {
    file_format: &FileFormat {
        id: 1_593,
        source_type: SourceType::Pronom,
        name: "RPM Package Manager file",
        extensions: &["rpm", "src.rpm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
