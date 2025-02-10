use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_7096031: FileType = FileType {
    file_format: &FileFormat {
        id: 7_096_031,
        source_type: SourceType::Wikidata,
        name: "Open Financial Connectivity",
        extensions: &["ofc"],
        media_types: &["text/ofc"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x4F, 0x46, 0x43, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
