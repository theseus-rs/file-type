use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862553: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_553,
        source_type: SourceType::Wikidata,
        name: "MegaStation module",
        extensions: &["ms"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4F, 0x4E, 0x47, 0x30, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
