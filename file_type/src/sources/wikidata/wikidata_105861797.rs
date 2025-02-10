use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861797: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_797,
        source_type: SourceType::Wikidata,
        name: "The 0ok Amazing Synth Tracker module",
        extensions: &["t0ast"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x30, 0x41, 0x53, 0x54, 0x01, 0x00, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
