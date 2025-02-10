use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856069: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_069,
        source_type: SourceType::Wikidata,
        name: "Dac-Easy Word Document",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x44, 0x41, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
