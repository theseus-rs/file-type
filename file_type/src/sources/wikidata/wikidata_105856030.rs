use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856030: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_030,
        source_type: SourceType::Wikidata,
        name: "IBM Document Content Architecture / Revisable Form Text",
        extensions: &["dca", "rft"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x05, 0xE1, 0x03, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
