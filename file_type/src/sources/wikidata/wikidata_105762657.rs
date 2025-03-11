use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762657: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_657,
        source_type: SourceType::Wikidata,
        name: "Albion game data archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x4C, 0x44, 0x30, 0x49, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
