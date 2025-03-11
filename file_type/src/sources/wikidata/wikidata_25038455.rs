use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25038455: FileType = FileType {
    file_format: &FileFormat {
        id: 25_038_455,
        source_type: SourceType::Wikidata,
        name: "Advanced Scientific Data Format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x41, 0x53, 0x44, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
