use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850618: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_618,
        source_type: SourceType::Wikidata,
        name: "Color Font Maker pattern (v1)",
        extensions: &["cfm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x46, 0x4D, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
