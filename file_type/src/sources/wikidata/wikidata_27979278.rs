use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979278: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_278,
        source_type: SourceType::Wikidata,
        name: "TheDraw Fonts File",
        extensions: &["tdf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x13, 0x54, 0x68, 0x65, 0x44, 0x72, 0x61, 0x77, 0x20, 0x46, 0x4F, 0x4E,
                        0x54, 0x53, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
