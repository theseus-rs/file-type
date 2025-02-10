use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861525: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_525,
        source_type: SourceType::Wikidata,
        name: "LIFE 3000 status",
        extensions: &["lif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x49, 0x46, 0x45, 0x20, 0x33, 0x30, 0x30, 0x30, 0x00, 0x56, 0x31,
                        0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
