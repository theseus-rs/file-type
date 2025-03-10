use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762710: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_710,
        source_type: SourceType::Wikidata,
        name: "PCjs Project XML machine configuration",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x6D, 0x61, 0x63, 0x68, 0x69, 0x6E, 0x65, 0x20, 0x69, 0x64, 0x3D,
                        0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
