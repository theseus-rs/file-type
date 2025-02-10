use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858750: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_750,
        source_type: SourceType::Wikidata,
        name: "FXG bitmap",
        extensions: &["fxg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDB, 0x46, 0x58, 0x47, 0x30, 0x31, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
