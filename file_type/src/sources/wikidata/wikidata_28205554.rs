use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205554: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_554,
        source_type: SourceType::Wikidata,
        name: "Nokia Logo Manager bitmap",
        extensions: &["nlm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x4C, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
