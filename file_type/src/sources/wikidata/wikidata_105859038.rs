use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859038: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_038,
        source_type: SourceType::Wikidata,
        name: "GTA: San Andreas save game (v1.00 PC mod)",
        extensions: &["b"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x83, 0xE5, 0xF3, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
