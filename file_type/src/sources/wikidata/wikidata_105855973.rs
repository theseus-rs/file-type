use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855973: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_973,
        source_type: SourceType::Wikidata,
        name: "MindReader Dictionary (v2.x)",
        extensions: &["dic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x52, 0x44, 0x49, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
