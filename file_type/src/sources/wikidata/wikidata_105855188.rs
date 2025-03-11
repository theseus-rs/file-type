use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855188: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_188,
        source_type: SourceType::Wikidata,
        name: "File Imploder compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4D, 0x50, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
