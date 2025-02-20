use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865110: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_110,
        source_type: SourceType::Wikidata,
        name: "InnovMetric Software Polygon Model",
        extensions: &["pol"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4F, 0x4C, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
