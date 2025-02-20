use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860266: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_266,
        source_type: SourceType::Wikidata,
        name: "SHELX output",
        extensions: &["res"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
