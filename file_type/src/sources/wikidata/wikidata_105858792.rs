use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858792: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_792,
        source_type: SourceType::Wikidata,
        name: "Red Sector Demo-Maker vector-ball object",
        extensions: &["bal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x41, 0x4C, 0x4C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
