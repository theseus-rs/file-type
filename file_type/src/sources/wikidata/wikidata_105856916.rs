use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856916: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_916,
        source_type: SourceType::Wikidata,
        name: "Golfstar Resource data",
        extensions: &["gsr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x6F, 0x6C, 0x66, 0x53, 0x74, 0x61, 0x72, 0x20, 0x52, 0x65, 0x73,
                        0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2C, 0x20,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
