use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850861: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_861,
        source_type: SourceType::Wikidata,
        name: "KVIrc Addon",
        extensions: &["kva"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x56, 0x50, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
