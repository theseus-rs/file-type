use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860435: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_435,
        source_type: SourceType::Wikidata,
        name: "Rob Northen Compression (type 1)",
        extensions: &["rnc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x4E, 0x43, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
