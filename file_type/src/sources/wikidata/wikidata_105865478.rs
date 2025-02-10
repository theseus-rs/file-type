use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865478: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_478,
        source_type: SourceType::Wikidata,
        name: "PowerBuilder trace Profile",
        extensions: &["pbp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x42, 0x54, 0x52, 0x41, 0x43, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
