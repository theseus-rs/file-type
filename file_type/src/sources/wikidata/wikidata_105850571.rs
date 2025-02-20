use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850571: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_571,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM MASK encrypted (v2.x)",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE8])],
                },
            }],
        }],
        related_formats: &[],
    },
};
