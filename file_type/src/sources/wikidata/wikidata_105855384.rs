use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855384: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_384,
        source_type: SourceType::Wikidata,
        name: "FMTracker Instruments",
        extensions: &["fmi"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x4D, 0x54, 0x49, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x6D, 0x65, 0x6E,
                        0x74, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
