use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861502: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_502,
        source_type: SourceType::Wikidata,
        name: "Sprint Layout Printed Circuit Design (v6.0)",
        extensions: &["lay"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x05, 0x33, 0xAA, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
