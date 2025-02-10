use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851703: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_703,
        source_type: SourceType::Wikidata,
        name: "PolyPlot Segments data (v3.0)",
        extensions: &["seg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x1D, 0x08, 0x08, 0x08, 0x50, 0x6F, 0x6C, 0x79, 0x70, 0x6C, 0x6F,
                        0x74, 0x20, 0x33, 0x2E, 0x30, 0x20, 0x53, 0x65, 0x67, 0x6D, 0x65, 0x6E,
                        0x74, 0x64, 0x61, 0x74, 0x65, 0x69, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
