use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857938: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_938,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine creature (v9.0)",
        extensions: &["cre"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x52, 0x45, 0x20, 0x56, 0x39, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
