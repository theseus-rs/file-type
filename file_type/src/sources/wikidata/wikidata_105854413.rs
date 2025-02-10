use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854413: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_413,
        source_type: SourceType::Wikidata,
        name: "ZOO compressed archive (strict)",
        extensions: &["zoo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDC, 0xA7, 0xC4, 0xFD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
