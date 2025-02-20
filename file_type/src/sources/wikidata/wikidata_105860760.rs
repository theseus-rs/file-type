use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860760: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_760,
        source_type: SourceType::Wikidata,
        name: "RPG Maker VX encrypted Archive (generic)",
        extensions: &["rgssad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x47, 0x53, 0x53, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
