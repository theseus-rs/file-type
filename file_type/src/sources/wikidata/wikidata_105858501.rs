use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858501: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_501,
        source_type: SourceType::Wikidata,
        name: "GEM bitmap (v3)",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x03, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
