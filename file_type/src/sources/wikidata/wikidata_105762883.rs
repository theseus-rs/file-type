use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762883: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_883,
        source_type: SourceType::Wikidata,
        name: "Panzer Elite Action game data archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x70, 0x31, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
