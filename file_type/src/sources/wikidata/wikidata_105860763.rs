use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860763: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_763,
        source_type: SourceType::Wikidata,
        name: "aSc Timetables timetable",
        extensions: &["roz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x00, 0x05, 0x41, 0x53, 0x43, 0x54, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
