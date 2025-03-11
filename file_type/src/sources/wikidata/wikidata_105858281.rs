use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858281: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_281,
        source_type: SourceType::Wikidata,
        name: "RISC OS AIF executable",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x11, 0x00, 0x00, 0xEF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
