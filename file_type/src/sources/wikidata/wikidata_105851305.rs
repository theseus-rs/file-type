use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851305: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_305,
        source_type: SourceType::Wikidata,
        name: "Windows 7 Task Scheduler job",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFE, 0x3C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
