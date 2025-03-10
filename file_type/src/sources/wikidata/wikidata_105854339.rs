use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854339: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_339,
        source_type: SourceType::Wikidata,
        name: "Ami Write printer font",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A, 0x25, 0xD2, 0x2B, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
