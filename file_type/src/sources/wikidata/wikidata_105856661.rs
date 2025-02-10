use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856661: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_661,
        source_type: SourceType::Wikidata,
        name: "Windows URL shortcut",
        extensions: &["url"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
