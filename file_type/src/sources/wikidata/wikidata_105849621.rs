use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849621: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_621,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM Maverick's C0DER protected",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE9])],
                },
            }],
        }],
        related_formats: &[],
    },
};
