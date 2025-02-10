use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854668: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_668,
        source_type: SourceType::Wikidata,
        name: "DJarc compressed archive",
        extensions: &["dja"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4A, 0x61, 0x72, 0x63])],
                },
            }],
        }],
        related_formats: &[],
    },
};
