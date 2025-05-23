use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850344: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_344,
        source_type: SourceType::Wikidata,
        name: "Crunch-Mania compressed data (std)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x72, 0x4D, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
