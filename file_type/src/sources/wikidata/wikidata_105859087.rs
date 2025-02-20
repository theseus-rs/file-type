use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859087: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_087,
        source_type: SourceType::Wikidata,
        name: "GTA: San Andreas save game (v1.01 PC mod)",
        extensions: &["b"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5E, 0x76, 0x45, 0x93])],
                },
            }],
        }],
        related_formats: &[],
    },
};
