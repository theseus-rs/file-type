use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856909: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_909,
        source_type: SourceType::Wikidata,
        name: "MapSource GPS Waypoint Database",
        extensions: &["gdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x73, 0x52, 0x63, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
