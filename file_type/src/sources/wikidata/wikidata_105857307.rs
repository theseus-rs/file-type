use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857307: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_307,
        source_type: SourceType::Wikidata,
        name: "Hi-MD Minidisc ATRAC3+ audio data container",
        extensions: &["hma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x54, 0x58, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
