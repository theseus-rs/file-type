use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858528: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_528,
        source_type: SourceType::Wikidata,
        name: "PrintFox/Pagefox bitmap (640x800)",
        extensions: &["bin", "pg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
