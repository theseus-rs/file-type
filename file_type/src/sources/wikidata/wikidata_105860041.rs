use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860041: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_041,
        source_type: SourceType::Wikidata,
        name: "FLIC FLC video",
        extensions: &["flc"],
        media_types: &["video/flc"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x12, 0xAF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
