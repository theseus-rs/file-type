use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860428: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_428,
        source_type: SourceType::Wikidata,
        name: "Titanfall 2 game data archive",
        extensions: &["rpak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x50, 0x61, 0x6B, 0x07, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
