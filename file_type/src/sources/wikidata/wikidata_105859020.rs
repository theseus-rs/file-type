use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_020,
        source_type: SourceType::Wikidata,
        name: "Personal Paint encrypted bitmap",
        extensions: &["pic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x52, 0x59, 0x50, 0x54, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
