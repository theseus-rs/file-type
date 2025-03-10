use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860394: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_394,
        source_type: SourceType::Wikidata,
        name: "Racelogic CAN data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x61, 0x63, 0x65, 0x6C, 0x6F, 0x67, 0x69, 0x63, 0x20, 0x43, 0x61,
                        0x6E, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                        0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
