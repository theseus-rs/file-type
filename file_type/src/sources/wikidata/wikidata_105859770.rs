use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859770: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_770,
        source_type: SourceType::Wikidata,
        name: "MTV video",
        extensions: &["mtv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4D, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
