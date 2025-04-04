use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858698: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_698,
        source_type: SourceType::Wikidata,
        name: "MGR bitmap (modern, 8bit aligned)",
        extensions: &["mgr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x79, 0x7A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
