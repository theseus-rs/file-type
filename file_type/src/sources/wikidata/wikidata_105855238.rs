use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855238: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_238,
        source_type: SourceType::Wikidata,
        name: "File Imploder compressed data (clone 5)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x44, 0x41, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
