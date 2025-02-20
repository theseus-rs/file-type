use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865429: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_429,
        source_type: SourceType::Wikidata,
        name: "Persona 4 game data archive",
        extensions: &["pac"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x57, 0x5F, 0x50, 0x41, 0x43, 0x4B, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
