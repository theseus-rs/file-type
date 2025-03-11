use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852699: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_699,
        source_type: SourceType::Wikidata,
        name: "SpecBAS keyboard layout",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x58, 0x4B, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
