use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861137: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_137,
        source_type: SourceType::Wikidata,
        name: "Radix Level Game Design",
        extensions: &["lgd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE7, 0xFE, 0xFF, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
