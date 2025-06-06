use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858731: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_731,
        source_type: SourceType::Wikidata,
        name: "MGR bitmap (old, 8-bit, 16-bit aligned, alt)",
        extensions: &["mgr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x79])],
                },
            }],
        }],
        related_formats: &[],
    },
};
