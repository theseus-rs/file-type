use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860758: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_758,
        source_type: SourceType::Wikidata,
        name: "Rathole compressed data",
        extensions: &["rhl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x48, 0x4C, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
