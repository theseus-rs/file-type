use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852118: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_118,
        source_type: SourceType::Wikidata,
        name: "32-Bit Sequencer Script (v1.2)",
        extensions: &["scr"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x63, 0x65, 0x72, 0x56, 0x31, 0x2E,
                        0x32, 0x2D, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
