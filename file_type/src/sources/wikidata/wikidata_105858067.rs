use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858067: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_067,
        source_type: SourceType::Wikidata,
        name: "Ivona voice",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x58, 0x56, 0x76, 0x6F, 0x78, 0x5F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
