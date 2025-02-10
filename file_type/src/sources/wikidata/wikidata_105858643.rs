use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858643: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_643,
        source_type: SourceType::Wikidata,
        name: "Bank Street Writer dictionary",
        extensions: &["bsw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x53, 0x57, 0x72, 0x69, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6F, 0x6D,
                        0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x64, 0x69, 0x63, 0x74,
                        0x69, 0x6F, 0x6E, 0x61, 0x72, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
