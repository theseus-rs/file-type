use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851479: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_479,
        source_type: SourceType::Wikidata,
        name: "C16 Tape image format",
        extensions: &["raw", "tap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x31, 0x36, 0x2D, 0x54, 0x41, 0x50, 0x45, 0x2D, 0x52, 0x41, 0x57,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
