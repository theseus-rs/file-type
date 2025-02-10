use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858876: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_876,
        source_type: SourceType::Wikidata,
        name: "Apple Binary 2 Library Utility archive",
        extensions: &["blu"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A, 0x47, 0x4C, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
