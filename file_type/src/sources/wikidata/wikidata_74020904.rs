use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_74020904: FileType = FileType {
    file_format: &FileFormat {
        id: 74_020_904,
        source_type: SourceType::Wikidata,
        name: "RAG-D bitmap",
        extensions: &["rag"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x41, 0x47, 0x2D, 0x44, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
