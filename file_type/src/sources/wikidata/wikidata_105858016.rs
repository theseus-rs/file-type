use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858016: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_016,
        source_type: SourceType::Wikidata,
        name: "HD-Copy disk image",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0x18, 0x07])],
                },
            }],
        }],
        related_formats: &[],
    },
};
