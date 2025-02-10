use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858864: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_864,
        source_type: SourceType::Wikidata,
        name: "MGR bitmap (old, 8-bit, 16-bit aligned)",
        extensions: &["mgr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7A, 0x79])],
                },
            }],
        }],
        related_formats: &[],
    },
};
