use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860039: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_039,
        source_type: SourceType::Wikidata,
        name: "DreamForge video",
        extensions: &["dfa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x46, 0x49, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
