use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858852: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_852,
        source_type: SourceType::Wikidata,
        name: "Ray Dream data",
        extensions: &["brw"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x44, 0x43, 0x20, 0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
