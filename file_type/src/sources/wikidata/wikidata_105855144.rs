use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855144: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_144,
        source_type: SourceType::Wikidata,
        name: "Spring Engine unit Info",
        extensions: &["fbi"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x55, 0x4E, 0x49, 0x54, 0x49, 0x4E, 0x46, 0x4F, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
