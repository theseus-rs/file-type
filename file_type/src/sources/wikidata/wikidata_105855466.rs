use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855466: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_466,
        source_type: SourceType::Wikidata,
        name: "Psion Font",
        extensions: &["fon"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4F, 0x4E, 0xE3, 0x30, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
