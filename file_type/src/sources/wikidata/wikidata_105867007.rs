use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867007: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_007,
        source_type: SourceType::Wikidata,
        name: "Rumble Fighter game data archive",
        extensions: &["nsz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x53, 0x5A, 0x6A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
