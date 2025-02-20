use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858363: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_363,
        source_type: SourceType::Wikidata,
        name: "QDOS executable",
        extensions: &["exe", "obj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0xFB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
