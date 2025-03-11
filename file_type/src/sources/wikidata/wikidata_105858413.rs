use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858413: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_413,
        source_type: SourceType::Wikidata,
        name: "Linux i/386 NMAGIC executable",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x08, 0x01, 0x64, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
