use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857439: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_439,
        source_type: SourceType::Wikidata,
        name: "Aladdin 4D Drawing (v2.x)",
        extensions: &["4d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4C, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
