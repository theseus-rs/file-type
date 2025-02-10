use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855072: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_072,
        source_type: SourceType::Wikidata,
        name: "Savage Warriors Animation",
        extensions: &["anm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x48, 0x43, 0x4B, 0x00, 0x01, 0x03, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
