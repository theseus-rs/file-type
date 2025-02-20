use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867055: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_055,
        source_type: SourceType::Wikidata,
        name: "SeeYou Waypoint",
        extensions: &["ndb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x20, 0x49, 0x4C, 0x45, 0x43, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
