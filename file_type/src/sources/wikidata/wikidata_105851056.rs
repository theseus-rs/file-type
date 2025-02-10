use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851056: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_056,
        source_type: SourceType::Wikidata,
        name: "DemoManiac Text",
        extensions: &["txt"],
        media_types: &["txt/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x45, 0x58, 0x54, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
