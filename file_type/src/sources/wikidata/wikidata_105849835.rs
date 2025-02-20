use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849835: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_835,
        source_type: SourceType::Wikidata,
        name: "MS Flight Simulator aircraft configuration file",
        extensions: &["cfg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x66, 0x6C, 0x74, 0x73, 0x69, 0x6D, 0x2E, 0x30, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
