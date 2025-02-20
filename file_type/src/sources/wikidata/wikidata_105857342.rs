use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857342: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_342,
        source_type: SourceType::Wikidata,
        name: "JRun Server Application",
        extensions: &["jsa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA2, 0xAB, 0x0B, 0xF0, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
