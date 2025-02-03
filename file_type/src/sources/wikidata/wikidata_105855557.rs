use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855557: FileFormat = FileFormat {
    id: 105_855_557,
    source_type: SourceType::Wikidata,
    name: "OpenIL/DevIL raster/animation format",
    extensions: &["oil"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
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
};
