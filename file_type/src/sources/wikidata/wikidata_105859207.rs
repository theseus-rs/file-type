use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859207: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_207,
        source_type: SourceType::Wikidata,
        name: "Axon Raw Format bitmap (big endian)",
        extensions: &["arf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0x41, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
