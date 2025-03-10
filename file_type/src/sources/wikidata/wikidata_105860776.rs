use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860776: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_776,
        source_type: SourceType::Wikidata,
        name: "Generic RIFX container (little-endian)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x46, 0x49, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
