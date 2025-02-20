use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856618: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_618,
        source_type: SourceType::Wikidata,
        name: "Fugawi 3 waypoint format",
        extensions: &["wpt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x33, 0x57, 0x50, 0x54, 0x2D, 0x2D, 0x2E, 0x2D, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
