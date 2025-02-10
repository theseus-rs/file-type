use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858328: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_328,
        source_type: SourceType::Wikidata,
        name: "Media Safe encrypted data",
        extensions: &["enc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x57, 0x45, 0x4E, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
