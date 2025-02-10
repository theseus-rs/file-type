use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851623: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_623,
        source_type: SourceType::Wikidata,
        name: "Wii TPL images container",
        extensions: &["tpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x20, 0xAF, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
