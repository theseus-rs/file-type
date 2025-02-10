use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856911: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_911,
        source_type: SourceType::Wikidata,
        name: "Crossword Puzzle",
        extensions: &["gpm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1A, 0x32, 0x30, 0x43, 0x57, 0x50, 0x55, 0x5A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
