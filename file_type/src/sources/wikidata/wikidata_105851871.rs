use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851871: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_871,
        source_type: SourceType::Wikidata,
        name: "Spatial Data Modelling Language (v1.0)",
        extensions: &["sdml"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2A, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20,
                        0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
