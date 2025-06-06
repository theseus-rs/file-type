use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856477: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_477,
        source_type: SourceType::Wikidata,
        name: "WordPerfect document (Amiga)",
        extensions: &["wp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x80])],
                },
            }],
        }],
        related_formats: &[],
    },
};
