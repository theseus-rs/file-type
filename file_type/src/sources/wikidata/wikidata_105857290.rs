use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857290: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_290,
        source_type: SourceType::Wikidata,
        name: "Human Machine Interfaces MIDI Format (rev.2)",
        extensions: &["hmi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x4D, 0x49, 0x2D, 0x4D, 0x49, 0x44, 0x49, 0x53, 0x4F, 0x4E, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
