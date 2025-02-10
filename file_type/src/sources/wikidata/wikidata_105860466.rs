use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860466: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_466,
        source_type: SourceType::Wikidata,
        name: "NASCAR Racing 3 Replay",
        extensions: &["rpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDD, 0x0F, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
