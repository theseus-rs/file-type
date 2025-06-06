use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856375: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_375,
        source_type: SourceType::Wikidata,
        name: "FSC Rater primary data",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x46, 0x53, 0x43, 0x20,
                        0x52, 0x61, 0x74, 0x65, 0x72, 0x27, 0x73, 0x20, 0x70, 0x72, 0x69, 0x6D,
                        0x61, 0x72, 0x79, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
