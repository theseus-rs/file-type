use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855941: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_941,
        source_type: SourceType::Wikidata,
        name: "DarkWave Studio module",
        extensions: &["dwp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x50, 0x57, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
