use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858377: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_377,
        source_type: SourceType::Wikidata,
        name: "Organize! Environment",
        extensions: &["env"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD3, 0xD0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
