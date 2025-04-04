use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858465: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_465,
        source_type: SourceType::Wikidata,
        name: "CyberAIDS infected Apple 2 executable",
        extensions: &["sys"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x13, 0x13, 0x13])],
                },
            }],
        }],
        related_formats: &[],
    },
};
