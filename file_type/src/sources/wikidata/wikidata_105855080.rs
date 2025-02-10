use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855080: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_080,
        source_type: SourceType::Wikidata,
        name: "GNAT Ada Library Information",
        extensions: &["ali"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x20, 0x22, 0x47, 0x4E, 0x41, 0x54, 0x20, 0x4C, 0x69, 0x62, 0x20,
                        0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
