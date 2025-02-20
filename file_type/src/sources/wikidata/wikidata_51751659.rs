use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51751659: FileType = FileType {
    file_format: &FileFormat {
        id: 51_751_659,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Show, version 3",
        extensions: &["sh3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x47, 0x42, 0x31, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
