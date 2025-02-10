use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854729: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_729,
        source_type: SourceType::Wikidata,
        name: "LZIP compressed archive",
        extensions: &["lz"],
        media_types: &["application/x-lzip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x5A, 0x49, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
