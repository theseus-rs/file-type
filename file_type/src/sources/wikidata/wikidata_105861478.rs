use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861478: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_478,
        source_type: SourceType::Wikidata,
        name: "Skunny Kart Library game data",
        extensions: &["lid"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4B, 0x55, 0x4E, 0x4E, 0x59, 0x20, 0x4B, 0x41, 0x52, 0x54, 0x20,
                        0x4C, 0x49, 0x42, 0x20, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
