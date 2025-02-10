use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865551: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_551,
        source_type: SourceType::Wikidata,
        name: "Pecom 64 program",
        extensions: &["pecom"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x2D, 0x36, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
