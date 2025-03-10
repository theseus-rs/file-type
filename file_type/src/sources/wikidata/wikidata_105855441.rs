use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855441: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_441,
        source_type: SourceType::Wikidata,
        name: "File Imploder compressed data (clone 1)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x54, 0x4E, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
