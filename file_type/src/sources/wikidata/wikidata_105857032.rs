use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857032: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_032,
        source_type: SourceType::Wikidata,
        name: "MS Age of Empires II: The Conquerors Expansion v1.0 Saved Game",
        extensions: &["gax"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEC, 0xBD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
