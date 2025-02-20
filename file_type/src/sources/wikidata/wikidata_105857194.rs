use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857194: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_194,
        source_type: SourceType::Wikidata,
        name: "PlayStation Hierarchical 3D Model Data",
        extensions: &["hmd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
