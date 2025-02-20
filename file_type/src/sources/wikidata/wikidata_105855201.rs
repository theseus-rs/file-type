use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855201: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_201,
        source_type: SourceType::Wikidata,
        name: "PolyPlot Font",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x16, 0x08, 0x08, 0x08, 0x50, 0x6F, 0x6C, 0x79, 0x70, 0x6C, 0x6F, 0x74,
                        0x20, 0x46, 0x6F, 0x6E, 0x74, 0x64, 0x61, 0x74, 0x65, 0x69, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
