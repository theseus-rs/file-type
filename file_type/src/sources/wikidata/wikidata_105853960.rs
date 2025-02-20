use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853960: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_960,
        source_type: SourceType::Wikidata,
        name: "mkwACT lossless compressed audio (v4)",
        extensions: &["mkw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x6B, 0x77, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
