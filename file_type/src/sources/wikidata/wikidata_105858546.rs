use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858546: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_546,
        source_type: SourceType::Wikidata,
        name: "Blizzard Picture (type 2)",
        extensions: &["blp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4C, 0x50, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
