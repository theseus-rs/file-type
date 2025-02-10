use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855526: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_526,
        source_type: SourceType::Wikidata,
        name: "6502 binary relocation format (v1)",
        extensions: &["o65"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x00, 0x6F, 0x36, 0x35, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
