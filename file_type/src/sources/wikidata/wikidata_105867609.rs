use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867609: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_609,
        source_type: SourceType::Wikidata,
        name: "Chess Assistant comments index",
        extensions: &["ndx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6E, 0x64, 0x65, 0x78, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x66,
                        0x6F, 0x72, 0x20, 0x63, 0x6F, 0x6D, 0x6D, 0x65, 0x6E, 0x74, 0x73, 0x20,
                        0x73, 0x75, 0x62, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x2E, 0x20, 0x20,
                        0x43, 0x68, 0x65, 0x73, 0x73, 0x20, 0x41, 0x73, 0x73, 0x69, 0x73, 0x74,
                        0x61, 0x6E, 0x74, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
