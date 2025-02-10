use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858790: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_790,
        source_type: SourceType::Wikidata,
        name: "Memotech MTX BASIC source",
        extensions: &["bas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF2, 0xF8, 0x59, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
