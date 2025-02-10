use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860768: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_768,
        source_type: SourceType::Wikidata,
        name: "Godot Resource data",
        extensions: &["res"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x43, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
