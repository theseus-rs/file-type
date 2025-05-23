use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_674,
        source_type: SourceType::Wikidata,
        name: "Alan v3 Compiled adventure",
        extensions: &["a3c"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4C, 0x41, 0x4E, 0x03, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
