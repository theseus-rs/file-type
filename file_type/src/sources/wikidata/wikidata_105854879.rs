use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854879: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_879,
        source_type: SourceType::Wikidata,
        name: "AppleWorks document (v2.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x7D, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
