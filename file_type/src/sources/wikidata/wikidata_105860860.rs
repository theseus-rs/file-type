use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860860: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_860,
        source_type: SourceType::Wikidata,
        name: "Taxman's Retro Engine SDK data",
        extensions: &["rsdk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x44, 0x4B, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
