use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853325: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_325,
        source_type: SourceType::Wikidata,
        name: "Spring Engine Tile",
        extensions: &["smt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x70, 0x72, 0x69, 0x6E, 0x67, 0x20, 0x74, 0x69, 0x6C, 0x65, 0x66,
                        0x69, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
