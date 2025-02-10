use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855722: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_722,
        source_type: SourceType::Wikidata,
        name: "Flashback Object",
        extensions: &["obj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE6, 0x00, 0x98, 0x03, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
