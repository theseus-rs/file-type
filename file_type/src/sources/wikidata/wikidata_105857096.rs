use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857096: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_096,
        source_type: SourceType::Wikidata,
        name: "DeScribe glossary",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x02, 0x00, 0x44, 0x65, 0x53, 0x63, 0x72, 0x69, 0x62, 0x65, 0x20, 0x47,
                        0x6C, 0x6F, 0x73, 0x73, 0x61, 0x72, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
