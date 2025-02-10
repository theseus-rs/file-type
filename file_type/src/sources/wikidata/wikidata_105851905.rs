use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851905: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_905,
        source_type: SourceType::Wikidata,
        name: "Smali assembly source",
        extensions: &["smali"],
        media_types: &["text/smali"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x63, 0x6C, 0x61, 0x73, 0x73, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
