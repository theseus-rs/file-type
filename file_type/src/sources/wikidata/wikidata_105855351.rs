use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855351: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_351,
        source_type: SourceType::Wikidata,
        name: "File Imploder compressed data (clone 2)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x44, 0x50, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
