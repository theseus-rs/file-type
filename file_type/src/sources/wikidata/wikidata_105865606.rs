use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865606: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_606,
        source_type: SourceType::Wikidata,
        name: "Casio Prizm add-in",
        extensions: &["g3a"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xAA, 0xAC, 0xBD, 0xAF, 0x90, 0x88, 0x9A, 0x8D, 0xD3, 0xFF, 0xFE, 0xFF,
                        0xFE, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
