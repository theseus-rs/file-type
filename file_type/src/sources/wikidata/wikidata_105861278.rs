use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861278: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_278,
        source_type: SourceType::Wikidata,
        name: "Need for Speed game data",
        extensions: &["lzc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x44, 0x4C, 0x5A, 0x02, 0x10, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
