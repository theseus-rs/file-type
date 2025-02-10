use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852414: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_414,
        source_type: SourceType::Wikidata,
        name: "Show.kit template",
        extensions: &["sks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x53, 0x4B, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
