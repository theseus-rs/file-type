use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856424: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_424,
        source_type: SourceType::Wikidata,
        name: "WordPerfect locked document (Amiga)",
        extensions: &["wp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xFF, 0x61, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
