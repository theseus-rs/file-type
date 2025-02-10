use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860838: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_838,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project 4.0 for DOS Resources",
        extensions: &["res"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x52, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
