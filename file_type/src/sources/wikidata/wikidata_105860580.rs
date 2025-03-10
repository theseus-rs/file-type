use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860580: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_580,
        source_type: SourceType::Wikidata,
        name: "Real 3D object (v1.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
