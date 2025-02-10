use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861888: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_888,
        source_type: SourceType::Wikidata,
        name: "Digital Tracker 4-channel module",
        extensions: &["mod"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x41, 0x30, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
