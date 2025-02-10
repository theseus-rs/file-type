use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857475: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_475,
        source_type: SourceType::Wikidata,
        name: "3D Slash model",
        extensions: &["3dslash"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x33, 0x44, 0x20, 0x53, 0x4C, 0x41, 0x53, 0x48, 0x30, 0x30, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
