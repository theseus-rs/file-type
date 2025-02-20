use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858819: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_819,
        source_type: SourceType::Wikidata,
        name: "ByteMap font format",
        extensions: &["bmf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE1, 0xE6, 0xD5, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
