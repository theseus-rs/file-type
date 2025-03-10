use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850433: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_433,
        source_type: SourceType::Wikidata,
        name: "Cyberware Digitizer Data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x79, 0x62, 0x65, 0x72, 0x77, 0x61, 0x72, 0x65, 0x20, 0x44, 0x69,
                        0x67, 0x69, 0x74, 0x69, 0x7A, 0x65, 0x72, 0x20, 0x44, 0x61, 0x74, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
