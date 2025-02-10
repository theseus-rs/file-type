use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_674,
        source_type: SourceType::Wikidata,
        name: "Axon Raw Format bitmap (little endian)",
        extensions: &["arf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x00, 0x41, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
