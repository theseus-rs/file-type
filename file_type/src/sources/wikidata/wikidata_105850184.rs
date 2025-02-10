use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850184: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_184,
        source_type: SourceType::Wikidata,
        name: "Team F1 Circuit data",
        extensions: &["cdr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x4C, 0x69, 0x63, 0x68, 0x74, 0x71, 0x75, 0x65, 0x6C, 0x6C, 0x65,
                        0x20, 0x62, 0x65, 0x69, 0x20, 0x30, 0x2C, 0x30, 0x2C, 0x31, 0x30, 0x30,
                        0x30, 0x2C, 0x20, 0x6E, 0x69, 0x63, 0x68, 0x74, 0x20, 0x6F, 0x72, 0x69,
                        0x65, 0x6E, 0x74, 0x69, 0x65, 0x72, 0x74, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
