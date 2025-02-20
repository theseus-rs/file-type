use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860679: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_679,
        source_type: SourceType::Wikidata,
        name: "RViz workspace",
        extensions: &["rviz"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x61, 0x6E, 0x65, 0x6C, 0x73, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
