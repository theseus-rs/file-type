use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853132: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_132,
        source_type: SourceType::Wikidata,
        name: "Bars and Pipes Professional song",
        extensions: &["song"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x52, 0x50, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
