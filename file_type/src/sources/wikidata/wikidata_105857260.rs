use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857260: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_260,
        source_type: SourceType::Wikidata,
        name: "HP Raster Transfer Language",
        extensions: &["rtl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1B, 0x45, 0x1B, 0x25, 0x30, 0x42, 0x42, 0x50, 0x31, 0x2C, 0x22, 0x62,
                        0x69, 0x74, 0x6D, 0x61, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
