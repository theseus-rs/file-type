use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849619: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_619,
        source_type: SourceType::Wikidata,
        name: "Common Ground Digital Paper document",
        extensions: &["dp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x47, 0x44, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
