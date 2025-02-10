use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851095: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_095,
        source_type: SourceType::Wikidata,
        name: "TimeZone data",
        extensions: &["tz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x5A, 0x69, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
