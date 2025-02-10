use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857323: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_323,
        source_type: SourceType::Wikidata,
        name: "Jnes save state",
        extensions: &["jst"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x53, 0x54, 0x60])],
                },
            }],
        }],
        related_formats: &[],
    },
};
