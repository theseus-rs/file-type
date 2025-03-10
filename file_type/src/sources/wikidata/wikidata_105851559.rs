use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851559: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_559,
        source_type: SourceType::Wikidata,
        name: "TSComp compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x65, 0x5D, 0x13, 0x8C, 0x08, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
