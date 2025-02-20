use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865380: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_380,
        source_type: SourceType::Wikidata,
        name: "PuavoHard Intro Music Composer Instrument (v3)",
        extensions: &["phpimci"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0C, 0x00, 0x00, 0x00, 0x70, 0x00, 0x68, 0x00, 0x70, 0x00, 0x69, 0x00,
                        0x6D, 0x00, 0x63, 0x00, 0x69, 0x00, 0x20, 0x00, 0x76, 0x00, 0x33, 0x00,
                        0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
