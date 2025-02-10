use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851376: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_376,
        source_type: SourceType::Wikidata,
        name: "IFF Cruncher compressed data",
        extensions: &["cr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x54, 0x57, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
