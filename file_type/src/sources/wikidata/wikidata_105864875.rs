use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864875: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_875,
        source_type: SourceType::Wikidata,
        name: "Garmin PCX5 waypoint file",
        extensions: &["wpt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
