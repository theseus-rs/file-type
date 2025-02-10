use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861124: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_124,
        source_type: SourceType::Wikidata,
        name: "LZS/Stac compressed data",
        extensions: &["lzs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x54, 0x61, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
