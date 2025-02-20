use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856622: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_622,
        source_type: SourceType::Wikidata,
        name: "WAD3 game data",
        extensions: &["wad"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x41, 0x44, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
