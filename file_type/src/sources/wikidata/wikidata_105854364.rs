use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_364,
        source_type: SourceType::Wikidata,
        name: "SimAnt saved game (Amiga)",
        extensions: &["ant"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x41, 0x47, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
