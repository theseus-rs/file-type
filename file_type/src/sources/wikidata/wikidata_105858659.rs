use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858659: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_659,
        source_type: SourceType::Wikidata,
        name: "BFLT - Binary Flat Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x62, 0x46, 0x4C, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
