use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850127: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_127,
        source_type: SourceType::Wikidata,
        name: "Weather Analytics data",
        extensions: &["csv"],
        media_types: &["text/csv"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x69, 0x74, 0x65, 0x49, 0x64, 0x2C, 0x57, 0x73, 0x57, 0x61, 0x49,
                        0x64, 0x78, 0x2C, 0x44, 0x61, 0x74, 0x65, 0x48, 0x72, 0x47, 0x6D, 0x74,
                        0x2C, 0x44, 0x61, 0x74, 0x65, 0x48, 0x72, 0x4C, 0x77, 0x74, 0x2C, 0x54,
                        0x73, 0x66, 0x63, 0x5F, 0x46, 0x2C, 0x54, 0x64, 0x65, 0x77, 0x5F, 0x46,
                        0x2C, 0x54, 0x77, 0x65, 0x74, 0x5F, 0x46, 0x2C, 0x52, 0x68, 0x5F, 0x50,
                        0x43, 0x54, 0x2C, 0x50, 0x73, 0x66, 0x63, 0x5F, 0x4D, 0x42, 0x2C, 0x43,
                        0x6C, 0x64, 0x43, 0x6F, 0x76, 0x5F, 0x50, 0x43, 0x54, 0x2C, 0x54, 0x77,
                        0x63, 0x5F, 0x46, 0x2C, 0x54, 0x61, 0x70, 0x70, 0x5F, 0x46, 0x2C, 0x53,
                        0x70, 0x64, 0x5F, 0x4B, 0x54, 0x53, 0x2C, 0x44, 0x69, 0x72, 0x5F, 0x44,
                        0x45, 0x47, 0x2C, 0x50, 0x63, 0x70, 0x50, 0x72, 0x65, 0x76, 0x48, 0x72,
                        0x5F, 0x49, 0x4E, 0x2C, 0x44, 0x6E, 0x53, 0x6F, 0x6C, 0x5F, 0x57, 0x73,
                        0x71, 0x4D, 0x2C, 0x44, 0x69, 0x66, 0x66, 0x48, 0x6F, 0x72, 0x7A, 0x5F,
                        0x57, 0x73, 0x71, 0x4D, 0x2C, 0x44, 0x69, 0x72, 0x4E, 0x6F, 0x72, 0x6D,
                        0x49, 0x72, 0x5F, 0x57, 0x73, 0x71, 0x4D, 0x2C, 0x53, 0x70, 0x64, 0x5F,
                        0x4D, 0x50, 0x48, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
