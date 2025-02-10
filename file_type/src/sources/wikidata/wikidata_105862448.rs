use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862448: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_448,
        source_type: SourceType::Wikidata,
        name: "MegaCAD Library",
        extensions: &["mtl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x45, 0x47, 0x41, 0x4C, 0x49, 0x42, 0x20, 0x30, 0x30, 0x34, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x00,
                        0x20, 0x00, 0x00, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
