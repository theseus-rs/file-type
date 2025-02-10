use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851552: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_552,
        source_type: SourceType::Wikidata,
        name: "Text Plus document (v3.0)",
        extensions: &["txp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x33, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
