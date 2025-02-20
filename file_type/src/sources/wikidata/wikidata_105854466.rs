use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854466: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_466,
        source_type: SourceType::Wikidata,
        name: "asciicast (v2)",
        extensions: &["cast"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
