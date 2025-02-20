use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859170: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_170,
        source_type: SourceType::Wikidata,
        name: "Mallard BASIC tokenized source (v1.11)",
        extensions: &["bas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFC, 0x03, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
