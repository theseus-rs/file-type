use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861927: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_927,
        source_type: SourceType::Wikidata,
        name: "Cisco IOS mzip compressed data",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x5A, 0x49, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
