use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856178: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_178,
        source_type: SourceType::Wikidata,
        name: "FL Studio Drum Pattern",
        extensions: &["dmptrn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x4D, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
