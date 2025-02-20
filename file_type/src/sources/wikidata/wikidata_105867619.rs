use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867619: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_619,
        source_type: SourceType::Wikidata,
        name: "WordPerfect Notebook",
        extensions: &["nb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0E, 0x0A, 0xC0, 0xC0, 0xC0, 0x01, 0x50, 0xC0, 0x9E, 0x30, 0x31, 0x20,
                        0x9D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
