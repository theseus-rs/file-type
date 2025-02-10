use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856712: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_712,
        source_type: SourceType::Wikidata,
        name: "Sprint User Interface",
        extensions: &["ui"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0C, 0xD4])],
                },
            }],
        }],
        related_formats: &[],
    },
};
