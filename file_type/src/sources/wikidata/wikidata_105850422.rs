use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850422: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_422,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM DS-COM Crypt protected (v1.27)",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xBB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
