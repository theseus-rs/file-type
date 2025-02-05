use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860397: FileFormat = FileFormat {
    id: 105_860_397,
    source_type: SourceType::Wikidata,
    name: "Raster-Vector Hybrid Drawing",
    extensions: &["rvd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x50, 0x2D, 0x52, 0x56, 0x44, 0x00, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
