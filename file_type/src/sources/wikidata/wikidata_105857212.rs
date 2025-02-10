use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857212: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_212,
        source_type: SourceType::Wikidata,
        name: "Heredis tree (v12)",
        extensions: &["ha12"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x52, 0x42, 0x52, 0x48, 0x45, 0x41, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
