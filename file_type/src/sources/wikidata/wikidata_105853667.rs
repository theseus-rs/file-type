use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853667: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_667,
        source_type: SourceType::Wikidata,
        name: "Advanced Digital Audio lossless compressed audio",
        extensions: &["ada"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x44, 0x41, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
