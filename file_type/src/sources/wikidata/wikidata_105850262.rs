use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850262: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_262,
        source_type: SourceType::Wikidata,
        name: "Crunch-Mania compressed data (std-sampled)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x72, 0x6D, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
