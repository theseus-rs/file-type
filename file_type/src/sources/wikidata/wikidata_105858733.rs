use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858733: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_733,
        source_type: SourceType::Wikidata,
        name: "ERDAS Image bitmap (v7.x)",
        extensions: &["gis", "lan"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x45, 0x41, 0x44, 0x37])],
                },
            }],
        }],
        related_formats: &[],
    },
};
