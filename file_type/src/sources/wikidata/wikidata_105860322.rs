use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860322: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_322,
        source_type: SourceType::Wikidata,
        name: "Build Engine RFF encrypted container",
        extensions: &["rff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x46, 0x46, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
