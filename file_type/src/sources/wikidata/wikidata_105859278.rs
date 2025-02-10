use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859278: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_278,
        source_type: SourceType::Wikidata,
        name: "Amazon Kindle Update Package",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x30, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
