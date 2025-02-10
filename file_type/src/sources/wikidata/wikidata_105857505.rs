use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857505: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_505,
        source_type: SourceType::Wikidata,
        name: "3D World Studio model",
        extensions: &["3dw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x44, 0x57, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
