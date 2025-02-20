use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852106: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_106,
        source_type: SourceType::Wikidata,
        name: "Yu-Gi-Oh! Duel Monsters World Championship savegame",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x75, 0x47, 0x69, 0x57, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
