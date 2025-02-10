use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855443: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_443,
        source_type: SourceType::Wikidata,
        name: "Flasm disassembled Flash ActionScript bytecode",
        extensions: &["flm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x6F, 0x76, 0x69, 0x65, 0x20, 0x27])],
                },
            }],
        }],
        related_formats: &[],
    },
};
