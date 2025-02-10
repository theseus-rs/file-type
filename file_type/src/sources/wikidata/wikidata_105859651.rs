use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859651: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_651,
        source_type: SourceType::Wikidata,
        name: "Visual CertExam Suite Exam file",
        extensions: &["vce"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x85, 0xA8, 0x01, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
