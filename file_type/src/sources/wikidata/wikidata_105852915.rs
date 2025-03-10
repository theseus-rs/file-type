use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852915: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_915,
        source_type: SourceType::Wikidata,
        name: "Sfx Document Template Directory",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x66, 0x78, 0x20, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74,
                        0x20, 0x54, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x20, 0x44, 0x69,
                        0x72, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x79, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
