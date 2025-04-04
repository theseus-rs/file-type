use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855635: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_635,
        source_type: SourceType::Wikidata,
        name: "OMAX Make tool path data",
        extensions: &["omx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6E, 0x20, 0x4F,
                        0x4D, 0x41, 0x58, 0x20, 0x28, 0x2E, 0x4F, 0x4D, 0x58, 0x29, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x2E, 0x20, 0x20, 0x44, 0x6F, 0x20, 0x6E, 0x6F, 0x74,
                        0x20, 0x6D, 0x6F, 0x64, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
                        0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x32, 0x20, 0x6C, 0x69, 0x6E, 0x65,
                        0x73, 0x20, 0x6F, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69,
                        0x6C, 0x65, 0x2E, 0x20, 0x46, 0x6F, 0x72, 0x20, 0x69, 0x6E, 0x66, 0x6F,
                        0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x6F, 0x6E, 0x20, 0x74,
                        0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x66, 0x6F, 0x72,
                        0x6D, 0x61, 0x74, 0x20, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x63, 0x74, 0x20,
                        0x73, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x65, 0x6E, 0x67, 0x69,
                        0x6E, 0x65, 0x65, 0x72, 0x69, 0x6E, 0x67, 0x40, 0x6F, 0x6D, 0x61, 0x78,
                        0x2E, 0x63, 0x6F, 0x6D, 0x20, 0x6F, 0x72, 0x20, 0x76, 0x69, 0x73, 0x69,
                        0x74, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77,
                        0x2E, 0x6F, 0x6D, 0x61, 0x78, 0x2E, 0x63, 0x6F, 0x6D, 0x2E, 0x0D, 0x0A,
                        0x32, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
