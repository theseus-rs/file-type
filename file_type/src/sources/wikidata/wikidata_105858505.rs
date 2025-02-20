use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858505: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_505,
        source_type: SourceType::Wikidata,
        name: "MSX BASIC Graphics bitmap (screen 2)",
        extensions: &["grp", "sc2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x00, 0x00, 0xFF, 0x37, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
