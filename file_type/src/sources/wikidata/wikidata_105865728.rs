use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865728: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_728,
        source_type: SourceType::Wikidata,
        name: "Password Tracker Deluxe Tracking List",
        extensions: &["pwt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x54, 0x43, 0x52, 0x52, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
