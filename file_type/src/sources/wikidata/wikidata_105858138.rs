use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858138: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_138,
        source_type: SourceType::Wikidata,
        name: "PDP-8 core memory image",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x21, 0x70, 0x64, 0x70, 0x38, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
