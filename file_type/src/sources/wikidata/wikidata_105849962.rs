use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849962: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_962,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro Clip art",
        extensions: &["clp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x52, 0x54, 0x4C, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
