use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855557: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_557,
        source_type: SourceType::Wikidata,
        name: "OpenIL/DevIL raster/animation format",
        extensions: &["oil"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x49, 0x4C, 0x00, 0x71, 0x3D, 0x69, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
