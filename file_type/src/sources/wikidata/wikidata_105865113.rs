use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865113: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_113,
        source_type: SourceType::Wikidata,
        name: "PhotoFiltre path",
        extensions: &["pfv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x68, 0x6F, 0x74, 0x6F, 0x46, 0x69, 0x6C, 0x74, 0x72, 0x65, 0x20,
                        0x50, 0x61, 0x74, 0x68,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
