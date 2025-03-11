use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856095: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_095,
        source_type: SourceType::Wikidata,
        name: "DIET compressed data (v1.20)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x9D, 0x89, 0x64, 0x6C, 0x7A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
