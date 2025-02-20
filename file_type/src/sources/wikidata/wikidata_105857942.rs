use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857942: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_942,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine region/map (v1.x)",
        extensions: &["wed"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x45, 0x44, 0x20, 0x56, 0x31, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
