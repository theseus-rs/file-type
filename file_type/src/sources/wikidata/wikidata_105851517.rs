use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851517: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_517,
        source_type: SourceType::Wikidata,
        name: "C64 Tape image format (v1-updated)",
        extensions: &["raw", "tap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x36, 0x34, 0x2D, 0x54, 0x41, 0x50, 0x45, 0x2D, 0x52, 0x41, 0x57,
                        0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
