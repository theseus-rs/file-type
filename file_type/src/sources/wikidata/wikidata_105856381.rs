use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856381: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_381,
        source_type: SourceType::Wikidata,
        name: "UCDOS Print Define",
        extensions: &["def"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x43, 0x44, 0x4F, 0x53, 0x20, 0x50, 0x72, 0x69, 0x6E, 0x74, 0x20,
                        0x44, 0x65, 0x66, 0x69, 0x6E, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
