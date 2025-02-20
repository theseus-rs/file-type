use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860213: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_213,
        source_type: SourceType::Wikidata,
        name: "RandyTab guitar tablature",
        extensions: &["rtab"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x05, 0x32, 0x52, 0x54, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
