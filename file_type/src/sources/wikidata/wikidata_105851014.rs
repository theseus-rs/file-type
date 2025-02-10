use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851014: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_014,
        source_type: SourceType::Wikidata,
        name: "Timeline schedule (v2.0)",
        extensions: &["t@0"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0xF0, 0x08, 0x00, 0x34, 0x12, 0x78, 0x56, 0x54, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
