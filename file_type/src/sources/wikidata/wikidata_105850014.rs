use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850014: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_014,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM Crypt (Alex) encrypted (v1.0)",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE9])],
                },
            }],
        }],
        related_formats: &[],
    },
};
