use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849275: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_275,
        source_type: SourceType::Wikidata,
        name: "TrainController Railroad",
        extensions: &["yrr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0xD1, 0x5D, 0x71, 0x4E, 0xDA, 0xB5, 0xA3,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
