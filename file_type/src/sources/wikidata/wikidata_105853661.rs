use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853661: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_661,
        source_type: SourceType::Wikidata,
        name: "mkwACT lossless compressed audio (v3)",
        extensions: &["mkw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x6B, 0x77, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
