use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859049: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_049,
        source_type: SourceType::Wikidata,
        name: "BMW TIS grayscale bitmap",
        extensions: &["itw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x54, 0x57, 0x5F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
