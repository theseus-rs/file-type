use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855212: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_212,
        source_type: SourceType::Wikidata,
        name: "Stunt Island Film",
        extensions: &["flm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x49, 0x4C, 0x4D, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
