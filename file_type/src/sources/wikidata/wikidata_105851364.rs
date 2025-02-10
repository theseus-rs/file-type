use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_364,
        source_type: SourceType::Wikidata,
        name: "SDLTRS State Save",
        extensions: &["t8s"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x6C, 0x64, 0x74, 0x72, 0x73, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65,
                        0x20, 0x53, 0x61, 0x76, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
