use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855952: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_952,
        source_type: SourceType::Wikidata,
        name: "Palm Zire Photo database",
        extensions: &["db"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x42, 0x46, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
