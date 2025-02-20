use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865167: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_167,
        source_type: SourceType::Wikidata,
        name: "Red Faction 2 game data",
        extensions: &["peg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x45, 0x4B, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
