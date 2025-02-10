use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867015: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_015,
        source_type: SourceType::Wikidata,
        name: "NeoDesk icon (compressed)",
        extensions: &["nic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x4E, 0x49, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
