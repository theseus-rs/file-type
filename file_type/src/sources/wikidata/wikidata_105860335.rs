use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860335: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_335,
        source_type: SourceType::Wikidata,
        name: "Real 3D materials (v1.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
