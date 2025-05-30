use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859667: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_667,
        source_type: SourceType::Wikidata,
        name: "Presto! VideoWorks Project (v6)",
        extensions: &["vwp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x16, 0x4E, 0x65, 0x77, 0x53, 0x6F, 0x66, 0x74, 0x20, 0x56, 0x57, 0x6F,
                        0x72, 0x6B, 0x73, 0x36, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x14,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
