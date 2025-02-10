use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861001: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_001,
        source_type: SourceType::Wikidata,
        name: "Lua 5.2 bytecode",
        extensions: &["out"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x4C, 0x75, 0x61, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
