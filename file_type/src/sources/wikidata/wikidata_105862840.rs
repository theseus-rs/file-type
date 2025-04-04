use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862840: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_840,
        source_type: SourceType::Wikidata,
        name: "Musik-Trainer Notation",
        extensions: &["mtn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x20, 0x54, 0x52, 0x41, 0x49, 0x4E, 0x45,
                        0x52, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
