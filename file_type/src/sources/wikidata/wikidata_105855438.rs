use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855438: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_438,
        source_type: SourceType::Wikidata,
        name: "File Imploder compressed data (clone 9)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x44, 0x43, 0x39])],
                },
            }],
        }],
        related_formats: &[],
    },
};
