use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858123: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_123,
        source_type: SourceType::Wikidata,
        name: "IBM Writing Assistant document (v1.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x00, 0x06, 0x00, 0x06, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
