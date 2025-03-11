use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862026: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_026,
        source_type: SourceType::Wikidata,
        name: "MovieSetter project (v1.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x78, 0x4D, 0x6F, 0x76, 0x69, 0x65, 0x53, 0x65, 0x74, 0x74, 0x65, 0x72,
                        0x20, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
