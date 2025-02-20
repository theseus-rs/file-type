use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858368: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_368,
        source_type: SourceType::Wikidata,
        name: "LightWave Envelope data",
        extensions: &["env"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x57, 0x45, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
