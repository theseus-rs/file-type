use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852210: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_210,
        source_type: SourceType::Wikidata,
        name: "Accent DemoMaker Sequence",
        extensions: &["seq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA9, 0xD1, 0x6A, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
