use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861669: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_669,
        source_type: SourceType::Wikidata,
        name: "RPG Maker 2000/2003 Map",
        extensions: &["lmu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x4C, 0x63, 0x66, 0x4D, 0x61, 0x70, 0x55, 0x6E, 0x69, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
