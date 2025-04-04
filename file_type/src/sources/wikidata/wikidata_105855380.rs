use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855380: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_380,
        source_type: SourceType::Wikidata,
        name: "Fisher and Paykel Icon Data Format",
        extensions: &["fph"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x32, 0x30, 0x31, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
