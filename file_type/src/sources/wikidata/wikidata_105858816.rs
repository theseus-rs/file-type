use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858816: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_816,
        source_type: SourceType::Wikidata,
        name: "MAKI v1-b bitmap",
        extensions: &["mki"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x4B, 0x49, 0x30, 0x31, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
