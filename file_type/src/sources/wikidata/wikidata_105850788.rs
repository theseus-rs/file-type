use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850788: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_788,
        source_type: SourceType::Wikidata,
        name: "Reflections camera",
        extensions: &["kam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x4B, 0x00, 0x41, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
