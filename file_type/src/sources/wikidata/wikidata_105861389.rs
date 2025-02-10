use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861389: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_389,
        source_type: SourceType::Wikidata,
        name: "Lua 4.0 bytecode",
        extensions: &["out"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x4C, 0x75, 0x61, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
