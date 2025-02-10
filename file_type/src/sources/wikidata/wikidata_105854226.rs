use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854226: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_226,
        source_type: SourceType::Wikidata,
        name: "AMOS Banks group",
        extensions: &["abs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x6D, 0x42, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
