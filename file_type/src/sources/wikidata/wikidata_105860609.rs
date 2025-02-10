use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860609: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_609,
        source_type: SourceType::Wikidata,
        name: "Random Access Compression format",
        extensions: &["rac"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0xC3, 0x63])],
                },
            }],
        }],
        related_formats: &[],
    },
};
