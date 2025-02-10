use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858572: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_572,
        source_type: SourceType::Wikidata,
        name: "MGR bitmap (modern, squeezed)",
        extensions: &["mgr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x79, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
