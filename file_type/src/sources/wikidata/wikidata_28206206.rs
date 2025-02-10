use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206206: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_206,
        source_type: SourceType::Wikidata,
        name: "Graph Saurus SR5",
        extensions: &["sr5"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x00, 0x00, 0x00, 0x6A, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
