use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849915: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_915,
        source_type: SourceType::Wikidata,
        name: "Calamus Raster Information",
        extensions: &["cri"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x4D, 0x43, 0x20, 0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x20,
                        0x46, 0x47, 0x45, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
