use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861520: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_520,
        source_type: SourceType::Wikidata,
        name: "Virtual Villagers data",
        extensions: &["ldw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6C, 0x64, 0x77, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
