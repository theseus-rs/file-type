use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860689: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_689,
        source_type: SourceType::Wikidata,
        name: "RPG Maker VX Project",
        extensions: &["rvproj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x50, 0x47, 0x56, 0x58, 0x20, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
