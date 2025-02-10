use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854522: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_522,
        source_type: SourceType::Wikidata,
        name: "Arena opening Book",
        extensions: &["abk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x41, 0x42, 0x4B, 0x70, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
