use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849853: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_853,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM DS-COM Crypt protected (v1.31)",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xBA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
