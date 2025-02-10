use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854714: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_714,
        source_type: SourceType::Wikidata,
        name: "bsc compressed data",
        extensions: &["bsc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x62, 0x73, 0x63, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
