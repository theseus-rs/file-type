use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859255: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_255,
        source_type: SourceType::Wikidata,
        name: "GTA: San Andreas save game (v1.01 PC)",
        extensions: &["b"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0xBE, 0x6E, 0x9A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
