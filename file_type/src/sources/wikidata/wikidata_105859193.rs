use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859193: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_193,
        source_type: SourceType::Wikidata,
        name: "Beasts and Bumpkins game data archive",
        extensions: &["box"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4F, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
