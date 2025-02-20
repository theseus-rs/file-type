use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851496: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_496,
        source_type: SourceType::Wikidata,
        name: "TECkit compiled mapping",
        extensions: &["tec"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7A, 0x51, 0x6D, 0x70, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
