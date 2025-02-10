use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854032: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_032,
        source_type: SourceType::Wikidata,
        name: "WordPerfect Printer info",
        extensions: &["all"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6A, 0x62, 0x68, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
