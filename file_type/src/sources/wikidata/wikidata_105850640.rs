use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850640: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_640,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM ComProtector encrypted (v1.0)",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE9])],
                },
            }],
        }],
        related_formats: &[],
    },
};
