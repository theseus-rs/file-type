use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850278: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_278,
        source_type: SourceType::Wikidata,
        name: "Cork compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x52, 0x4B, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
