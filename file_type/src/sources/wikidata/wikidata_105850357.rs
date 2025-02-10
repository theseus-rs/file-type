use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850357: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_357,
        source_type: SourceType::Wikidata,
        name: "HMI Sound Operating System Configuration",
        extensions: &["cfg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0D, 0x0A, 0x5B, 0x44, 0x49, 0x47, 0x49, 0x54, 0x41, 0x4C, 0x5D, 0x0D,
                        0x0A, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4E, 0x61, 0x6D, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
