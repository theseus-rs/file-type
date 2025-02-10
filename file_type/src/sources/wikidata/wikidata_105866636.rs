use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866636: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_636,
        source_type: SourceType::Wikidata,
        name: "PowerPacker compressed (v1.1)",
        extensions: &["pp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x31, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
