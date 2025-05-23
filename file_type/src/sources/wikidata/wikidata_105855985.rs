use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855985: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_985,
        source_type: SourceType::Wikidata,
        name: "Aegis Draw 2000 v2.x drawing",
        extensions: &["drawing"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x31, 0x30, 0x39, 0x31, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
