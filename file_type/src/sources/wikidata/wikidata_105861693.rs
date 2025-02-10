use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861693: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_693,
        source_type: SourceType::Wikidata,
        name: "NASCAR SimRacing game data archive",
        extensions: &["mas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x55, 0x42, 0x45, 0x4D, 0x41, 0x53, 0x34, 0x2E, 0x31, 0x30, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
