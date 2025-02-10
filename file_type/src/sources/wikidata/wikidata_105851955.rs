use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851955: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_955,
        source_type: SourceType::Wikidata,
        name: "Snzip compressed (comment-43 format)",
        extensions: &["snappy"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x06, 0x00, 0x73, 0x6E, 0x61, 0x70, 0x70, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
