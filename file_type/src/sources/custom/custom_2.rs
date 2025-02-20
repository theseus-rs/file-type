use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const CUSTOM_2: FileType = FileType {
    file_format: &FileFormat {
        id: 2,
        source_type: SourceType::Custom,
        name: "Apache Avro",
        extensions: &["avro"],
        media_types: &["application/vnd.apache.avro.file"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x62, 0x6A, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
