use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852144: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_144,
        source_type: SourceType::Wikidata,
        name: "Skale Tracker module",
        extensions: &["skm"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0xFE, 0xFF, 0x09, 0x00, 0x00, 0x00, 0x41, 0x4C, 0x49, 0x4D,
                        0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
