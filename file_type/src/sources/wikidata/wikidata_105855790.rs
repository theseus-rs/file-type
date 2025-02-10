use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855790: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_790,
        source_type: SourceType::Wikidata,
        name: "LocoScript Data (v1.x)",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x41, 0x54, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
