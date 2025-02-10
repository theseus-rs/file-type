use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857191: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_191,
        source_type: SourceType::Wikidata,
        name: "HOOPS 3D Stream Format",
        extensions: &["hsf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x3B, 0x20, 0x48, 0x53, 0x46, 0x20, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
