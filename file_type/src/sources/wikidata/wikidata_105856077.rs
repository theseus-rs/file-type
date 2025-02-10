use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856077: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_077,
        source_type: SourceType::Wikidata,
        name: "Graph Diagram",
        extensions: &["dia"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x72, 0x61, 0x70, 0x68, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
