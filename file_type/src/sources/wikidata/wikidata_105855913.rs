use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855913: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_913,
        source_type: SourceType::Wikidata,
        name: "Dolphin movie capture",
        extensions: &["dtm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x54, 0x4D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
