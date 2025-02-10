use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850657: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_657,
        source_type: SourceType::Wikidata,
        name: "Fullscreen Construction Kit bitmap (460x274)",
        extensions: &["kid"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x44, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
