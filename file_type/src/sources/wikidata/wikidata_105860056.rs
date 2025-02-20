use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860056: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_056,
        source_type: SourceType::Wikidata,
        name: "MegaPaint Vector format",
        extensions: &["vek"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0x56, 0x45, 0x4B, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
