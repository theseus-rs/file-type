use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52230534: FileType = FileType {
    file_format: &FileFormat {
        id: 52_230_534,
        source_type: SourceType::Wikidata,
        name: "Polynomial Texture Map",
        extensions: &["ptm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x54, 0x4D, 0x5F, 0x31, 0x2E, 0x32, 0x0A, 0x50, 0x54, 0x4D, 0x5F,
                        0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x5F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
