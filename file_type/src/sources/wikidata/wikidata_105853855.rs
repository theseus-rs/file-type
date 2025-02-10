use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853855: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_855,
        source_type: SourceType::Wikidata,
        name: "askSam Windows database",
        extensions: &["ask"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x73, 0x6B, 0x77])],
                },
            }],
        }],
        related_formats: &[],
    },
};
